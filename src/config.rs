#[derive(serde::Deserialize, Clone, Debug)]
pub struct TubeSettings {
    pub server: String,
    pub brokers: String,
    pub group: String,
    pub workers: i32,
}
