use futures::{
    future::{self, FutureExt},
    Future,
};
use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord},
    util::AsyncRuntime,
};
use std::time::{Duration, Instant};

pub struct SmolRuntime;

impl AsyncRuntime for SmolRuntime {
    type Delay = future::Map<async_io::Timer, fn(Instant)>;

    fn spawn<T>(task: T)
    where
        T: Future<Output = ()> + Send + 'static,
    {
        async_std::task::spawn(task);
    }

    fn delay_for(duration: Duration) -> Self::Delay {
        async_io::Timer::after(duration).map(|_| ())
    }
}

fn producer() -> FutureProducer {
    ClientConfig::new()
        .set("client.id", "oops")
        .set("bootstrap.servers", "kafka:9092")
        .set("queue.buffering.max.ms", "1000")
        .create()
        .unwrap()
}

#[async_std::main]
async fn main() {
    println!("Starting... ");
    let producer = producer();
    println!("... done!");
    loop {
        println!("-> ");
        let mut record = FutureRecord::<str, str>::to(&"test");
        record = record.payload("yuck");
        if producer
            .send_with_runtime::<SmolRuntime, _, _, _>(record, Duration::from_secs(0))
            .await
            .is_ok()
        {
            print!(".");
        } else {
            print!("!");
        }
    }
}
