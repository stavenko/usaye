use std::sync::Arc;

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api_error::Error,
    providers::{task_processor::TaskProcessor, types::TaskResult},
};

#[derive(thiserror::Error, Debug)]
#[allow(unused)]
pub enum GetTaskResultError {
    #[error("Internal server error")]
    InternalServerError,
}

impl From<GetTaskResultError> for Error {
    fn from(value: GetTaskResultError) -> Self {
        match value {
            GetTaskResultError::InternalServerError => Error {
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
) -> Result<Option<Result<TaskResult, Error>>, GetTaskResultError> {
    Ok(task_processor.get_task_result(id).await)
}
