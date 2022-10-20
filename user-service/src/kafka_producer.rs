use rdkafka::{error::KafkaError, producer::FutureProducer, ClientConfig};

pub fn create_kafka_producer(brokers: &str) -> Result<FutureProducer, KafkaError> {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "5000")
        .create()
}
