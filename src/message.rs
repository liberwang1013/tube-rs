#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub schema: String,
    pub method: String,
    pub metadata: super::metadata::Metadata,
    pub body: serde_json::Value,
}

#[derive(Debug, Clone, Default)]
pub struct MessageBuilder {
    pub method: String,
    pub body: serde_json::Value,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn method(&mut self, method: String) -> &mut Self {
        self.method = method;
        self
    }

    pub fn body(&mut self, body: serde_json::Value) -> &mut Self {
        self.body = body;
        self
    }

    pub fn build(&self) -> Message {
        Message {
            schema: "v1".into(),
            method: self.method.clone(),
            metadata: crate::Metadata {},
            body: self.body.clone(),
        }
    }
}

pub trait MessageTrait {
    fn method() -> String;
    fn body(&self) -> serde_json::Value;
}
