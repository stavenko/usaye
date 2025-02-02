use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    api_error::Error,
    providers::{self, task_processor::TaskProcessor},
};

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
    task_url: Url,
    #[serde(with = "humantime_serde", default)]
    delay: Duration,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
}

pub async fn command(
    Input { task_url, delay }: Input,
    task_processor: Arc<TaskProcessor>,
) -> Result<Task, AddTaskError> {
    let id = Uuid::new_v4();
    let task = providers::types::Task {
        id,
        url: task_url,
        delay,
    };
    task_processor.add_task(task).await;
    Ok(Task { id })
}
