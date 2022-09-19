use super::config::TubeClientSettings;
use crate::Message;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig as RdClientConfig,
};
use serde_json::Value;
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::Sender;

pub struct Client {
    brokers: String,
    server: String,
    tx: UnboundedSender<(Option<Sender<(i32, i64)>>, Value)>,
    rx: UnboundedReceiver<(Option<Sender<(i32, i64)>>, Value)>,
}

impl Client {
    pub fn new(conf: TubeClientSettings) -> Client {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<(
            Option<tokio::sync::oneshot::Sender<(i32, i64)>>,
            serde_json::Value,
        )>();
        Client {
            brokers: conf.brokers.clone(),
            server: conf.server.clone(),
            tx,
            rx,
        }
    }
    pub async fn run(&mut self) {
        let producer: FutureProducer = RdClientConfig::new()
            .set("bootstrap.servers", self.brokers.clone())
            .set("message.timeout.ms", "5000")
            .set("api.version.request", "true") //
            .create()
            .expect("Producer creation error");

        loop {
            tokio::select! {
                Some((sender, data)) = self.rx.recv() => {
                    let datab = serde_json::to_vec(&data).unwrap();
                    log::debug!("topic: {}, data: {}", &self.server, &data);
                    let digest = md5::compute(&datab);
                    let key = format!("{:x}", digest);
                    let produce_future = producer.send(
                        FutureRecord::to(&self.server).key(&key).payload(&datab),
                        Duration::from_secs(0),
                    );
                    match produce_future.await {
                        Ok(delivery) => {
                            println!("Sent: {:?}", delivery);
                            if let Some(tx) = sender {
                                if let Err(e) = tx.send(delivery) {
                                    log::warn!("failed to send kafka, error: {:?}", e);
                                }
                            }
                        },
                        Err((e, _)) => {
                            log::warn!("Error: {:?}", e);
                        },
                    }
                }
            };
        }
    }

    pub async fn send(&self, msg: Message) {
        log::debug!("msg: {:?}", &msg);
        let value = serde_json::to_value(msg).unwrap();
        let (sender, receiver) = tokio::sync::oneshot::channel::<(i32, i64)>();
        match self.tx.send((Some(sender), value)) {
            Ok(()) => match receiver.await {
                Ok(delivery) => log::debug!("parittion: {}, offset: {}", delivery.0, delivery.1),
                Err(e) => {
                    log::warn!("failed to send to kafka, error: {}", e);
                }
            },
            Err(e) => {
                log::warn!("failed to send message to kafka client: {}", e);
            }
        }
    }
}
