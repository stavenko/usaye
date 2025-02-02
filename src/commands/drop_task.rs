use std::sync::Arc;

use serde::Deserialize;
use uuid::Uuid;

use crate::{api_error::Error, providers::task_processor::TaskProcessor};

#[derive(thiserror::Error, Debug)]
#[allow(unused)]
pub enum AddTaskError {
    #[error("Internal server error")]
    InternalServerError,
}

impl From<AddTaskError> for Error {
    fn from(value: AddTaskError) -> Self {
        match value {
            AddTaskError::InternalServerError => Error {
                code: "InternalServerError".to_string(),
                message: value.to_string(),
            },
        }
    }
}

#[derive(Deserialize)]
pub struct Input {
    id: Uuid,
}

pub async fn command(
    Input { id }: Input,
    task_processor: Arc<TaskProcessor>,
) -> Result<(), AddTaskError> {
    task_processor.drop_task(id).await;
    Ok(())
}
