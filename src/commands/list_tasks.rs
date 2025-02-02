use std::sync::Arc;

use crate::{
    api_error::Error,
    providers::{task_processor::TaskProcessor, types::TaskView},
};

#[derive(thiserror::Error, Debug)]
#[allow(unused)]
pub enum ListTaskError {
    #[error("Internal server error")]
    InternalServerError,
}

impl From<ListTaskError> for Error {
    fn from(value: ListTaskError) -> Self {
        match value {
            ListTaskError::InternalServerError => Error {
                code: "InternalServerError".to_string(),
                message: value.to_string(),
            },
        }
    }
}

pub async fn command(task_processor: Arc<TaskProcessor>) -> Result<Vec<TaskView>, ListTaskError> {
    Ok(task_processor.list_tasks().await)
}
