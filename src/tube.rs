use super::handler::Handler;
use super::error::Error;
use super::message::Message as TubeMessage;
use super::{request::Request, response::Response};
use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};
use std::collections::HashMap;
use std::sync::Arc;

//type Handler = fn(Request) -> Response;

pub struct Tube<D: Send + Sync + 'static> {
    brokers: String,
    group_id: String,
    server: String,
    workers: i32,
    handlers: HashMap<String, Box<dyn Handler<D> + Sync + Send + 'static>>,
    shared_data: Arc<D>,
    error_handler: Option<Box<dyn Fn(Error)>>,
    response_handler: Option<Box<dyn Fn(Response)>>,
}

impl<D: Send + Sync + 'static> Tube<D> {
    pub fn new(server: String, brokers: String, group_id: String, data: D) -> Self {
        Self {
            server,
            brokers,
            group_id,
            workers: 3,
            shared_data: Arc::new(data),
            handlers: HashMap::new(),
            error_handler: None,
            response_handler: None,
        }
    }

    pub fn new_with_config(settings: super::TubeSettings, data: Arc<D>) -> Self {
        Self {
            server: settings.server,
            brokers: settings.brokers,
            group_id: settings.group,
            workers: settings.workers,
            shared_data: data,
            handlers: HashMap::new(),
            error_handler: None,
            response_handler: None,
        }
    }

    pub fn brokers(mut self, brokers: &str) -> Self {
        self.brokers = brokers.into();
        self
    }

    pub fn group(mut self, g: &str) -> Self {
        self.group_id = g.into();
        self
    }

    pub fn register_handler<H: Handler<D> + Sync + Send + 'static>(mut self, h: H) -> Self {
        self.handlers.insert(h.name(), Box::new(h));
        self
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", &self.group_id)
            .set("bootstrap.servers", &self.brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(&[&self.server])
            .expect("Can't subscribe to specified topic");

        (0..self.workers)
            .map(|_| {
                let processor = consumer
                    .stream()
                    .try_for_each(|borrowed_message| async move {
                        let payload = if let Some(p) = borrowed_message.payload() {
                            p
                        } else {
                            let offset = borrowed_message.offset();
                            log::warn!("tube warning: empty payload, offset: {}", &offset);
                            //return Err(Error::EmptyPayload(offset));
                            return Ok(());
                        };

                        let msg = if let Ok(msg) = serde_json::from_slice::<TubeMessage>(payload) {
                            msg
                        } else {
                            log::warn!("tube warning: wrong message format");
                            return Ok(());
                        };
                        let req = Request::from_message(msg, self.shared_data.clone());
                        let method = req.method.clone();
                        let handler = if let Some(h) = self.handlers.get(&method) {
                            h
                        } else {
                            log::warn!("tube warning: unsupport method({})", &method);
                            return Ok(());
                        };
                        match handler.call(req).await {
                            Ok(rsp) => {
                                if let Some(rsp_handler) = &self.response_handler {
                                    log::debug!("{:?}", &rsp);
                                    rsp_handler(rsp);
                                }
                            },
                            Err(e) => {
                                if let Some(err_handler) = &self.error_handler {
                                    log::debug!("{}", &e);
                                    err_handler(e);
                                }
                            }
                        }
                        Ok(())
                    });

                processor
            })
            .collect::<FuturesUnordered<_>>()
            .for_each(|_| async { () })
            .await;
        Ok(())
    }
}
