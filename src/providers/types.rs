use std::time::Duration;

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Clone)]
pub struct Task {
    pub id: Uuid,
    pub url: Url,
    pub delay: Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskResult {
    pub read_size: usize,
    pub status: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskView {
    pub id: Uuid,
    pub url: Url,
    pub task_status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Pending,
    Completed,
    Running,
}
