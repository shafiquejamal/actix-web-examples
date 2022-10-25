use actix::Addr;
use log::{info, warn};

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::Message;
use rdkafka::topic_partition_list::TopicPartitionList;

use crate::actor::{GlobalActor, GlobalActorMessage};
use crate::graphql::Person;

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
    pub global_actor_address: Addr<GlobalActor>,
}

impl IngestConsumer {
    pub fn new(
        brokers: &str,
        group_id: &str,
        topics: Vec<String>,
        global_actor_address: Addr<GlobalActor>,
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
        Ok(IngestConsumer {
            consumer,
            global_actor_address,
        })
    }

    pub async fn run(&self) {
        loop {
            match self.consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
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

                    // TODO: Handle the errors, reduce code duplication. Replace match with Map?  See: https://www.reddit.com/r/rust/comments/5mnj3y/which_has_better_performance_a_hashmap_or_a/
                    let request_id =
                        String::from_utf8(m.key().map(|x| x.to_vec()).unwrap()).unwrap();
                    let json: serde_json::Value =
                        serde_json::from_str(payload).expect("Could not parse payload");
                    let message_dto = json.get("response_message_dto").unwrap();

                    let response_type: &str = json.get("response_type").unwrap().as_str().unwrap();
                    match response_type {
                        "Person" => {
                            let data = message_dto.get("Person").unwrap().get("person").unwrap();
                            let person: Person = serde_json::from_value(data.clone()).unwrap();
                            let result = self
                                .global_actor_address
                                .clone()
                                .send(GlobalActorMessage::SendPersonMessage(request_id, person))
                                .await;
                            match result {
                                Err(e) => warn!("error sending person ({e}:?)"),
                                _ => (),
                            }
                        }
                        "Persons" => {
                            let data = message_dto.get("Persons").unwrap().get("persons").unwrap();
                            let persons: Vec<Person> =
                                serde_json::from_value(data.clone()).unwrap();
                            let result = self
                                .global_actor_address
                                .clone()
                                .send(GlobalActorMessage::SendPersonsMessage(request_id, persons))
                                .await;
                            match result {
                                Err(e) => warn!("error sending persons ({e}:?)"),
                                _ => (),
                            }
                        }
                        _ => {}
                    }
                    self.consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        }
    }
}
