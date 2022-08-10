use std::sync::Arc;
use super::message::Message;

#[derive(Debug, Clone)]
pub struct Request<D = ()> {
    pub schema: String,
    pub method: String,
    pub metadata: super::metadata::Metadata,
    pub body: serde_json::Value,
    pub data: Arc<D>,
}

impl <D> Request<D> {
    pub fn from_message(msg: Message, data: Arc<D>) -> Request<D> {
        Self {
            schema: msg.schema,
            method: msg.method,
            metadata: msg.metadata,
            body: msg.body,
            data
        }
    }
}
