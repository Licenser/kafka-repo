use std::time::Duration;

use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord},
};

fn producer() -> FutureProducer {
    ClientConfig::new()
        .set("client.id", "oops")
        .set("bootstrap.servers", "kafka:9092")
        .set("queue.buffering.max.ms", "1000")
        .create()
        .unwrap()
}

#[tokio::main]
async fn main() {
    println!("Starting... ");
    let producer = producer();
    println!("... done!");
    loop {
        println!("-> ");
        let mut record = FutureRecord::<str, str>::to(&"test");
        record = record.payload("yuck");
        if producer.send(record, Duration::from_secs(0)).await.is_ok() {
            print!(".");
        } else {
            print!("!");
        }
    }
}
