use std::time::Duration;

use log::{info, warn};

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::{Headers, Message};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::TopicPartitionList;

use crate::models::{
    Command, Id, Person, ResponseMessageDto, ResponseMessageDtoWrapper, ServiceRequest,
};

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
pub struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

pub struct IngestConsumer {
    pub consumer: LoggingConsumer,
    pub producer: FutureProducer,
}

const PUBLISH_TO: &str = "from_service";
impl IngestConsumer {
    pub fn new(
        brokers: String,
        group_id: String,
        topics: Vec<String>,
        producer: FutureProducer,
    ) -> Result<IngestConsumer, KafkaError> {
        let context = CustomContext;

        let consumer: LoggingConsumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            //.set("statistics.interval.ms", "30000")
            //.set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("Consumer creation failed");

        let topics: Vec<&str> = topics.iter().map(String::as_str).collect();
        consumer
            .subscribe(&topics)
            .expect("Can't subscribe to specified topics");
        Ok(IngestConsumer { consumer, producer })
    }

    pub async fn run(&self) {
        loop {
            match self.consumer.recv().await {
                Err(e) => {
                    warn!("Kafka error: {}", e);
                }
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };
                    info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                          m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                    if let Some(headers) = m.headers() {
                        for i in 0..headers.count() {
                            let header = headers.get(i).unwrap();
                            info!("  Header {:#?}: {:?}", header.0, header.1);
                        }
                    }
                    let ServiceRequest {
                        request_id,
                        command,
                    }: ServiceRequest = serde_json::from_str(payload).unwrap();
                    match command {
                        Command::GetPerson => {
                            let person = Person {
                                name: "Alice - default".to_string(),
                                id: Id {
                                    number: 1,
                                    department: "Executive".to_string(),
                                },
                            };
                            let response_message_dto = ResponseMessageDto::Person { person };
                            let message_dto_wrapper = ResponseMessageDtoWrapper {
                                request_id: request_id.clone(),
                                response_message_dto,
                                response_type: "Person".to_string(),
                            };
                            let payload = serde_json::json!(message_dto_wrapper).to_string();
                            self.producer
                                .send(
                                    FutureRecord::to(PUBLISH_TO)
                                        .payload(&payload)
                                        .key(&request_id.to_string()),
                                    Duration::from_secs(0),
                                )
                                .await
                                .unwrap();
                        }
                        Command::GetPersons => {
                            let alice = Person {
                                name: "Alice".to_string(),
                                id: Id {
                                    number: 1,
                                    department: "Executive".to_string(),
                                },
                            };
                            let bob = Person {
                                name: "Bob".to_string(),
                                id: Id {
                                    number: 2,
                                    department: "Finance".to_string(),
                                },
                            };
                            let charlie = Person {
                                name: "Charlie".to_string(),
                                id: Id {
                                    number: 3,
                                    department: "Operations".to_string(),
                                },
                            };
                            let persons = vec![alice, bob, charlie];
                            let response_message_dto = ResponseMessageDto::Persons { persons };
                            let message_dto_wrapper = ResponseMessageDtoWrapper {
                                request_id: request_id.clone(),
                                response_message_dto,
                                response_type: "Persons".to_string(),
                            };
                            let payload = serde_json::json!(message_dto_wrapper).to_string();
                            self.producer
                                .send(
                                    FutureRecord::to(PUBLISH_TO)
                                        .payload(&payload)
                                        .key(&request_id.to_string()),
                                    Duration::from_secs(0),
                                )
                                .await
                                .unwrap();
                        }
                    }

                    self.consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        }
    }
}
