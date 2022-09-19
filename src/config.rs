#[derive(serde::Deserialize, Clone, Debug)]
pub struct TubeSettings {
    pub server: String,
    pub brokers: String,
    pub group: String,
    pub workers: i32,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct TubeClientSettings {
    pub brokers: String,
    pub server: String,
}
