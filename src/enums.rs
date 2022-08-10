use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Default)]
pub enum SchemaVersion {
    #[default]
    V1,
}

impl std::fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let a = match self {
            Self::V1 => "v1",
        };
        write!(f, "{}", a)
    }
}

impl From<String> for SchemaVersion {
    fn from(s: String) -> Self {
        match s.as_str() {
            "v1" => Self::V1,
            _ => Self::V1
        }
    }
}



#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum StatusCode {
    Success = 0,
    #[default]
    Failed = -1,
}
