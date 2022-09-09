pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // common err
    #[error("EmptyPayload, offset: {0}")]
    EmptyPayload(i64),
    #[error("UnsupportMethod: {0}")]
    UnsupportMethod(String),
}

// impl Error {
//     fn error_code(&self) -> i32 {
//         match self {
//             // common error start from 10000
//             &Self::DatabaseError(_) => 10000,
//             &Self::InvalidatePage(_) => 10001,

//             &Self::TodoNotFound(_) => 20000,
//             &Self::WorkflowDenitionNotFound(_) => 21000,
//             &Self::JobNotFound(_) => 22000,
//             &Self::WorkflowNotFound(_) => 23000,
//         }
//     }
// }
