use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct TaskRunnerConfig {
    pub max_tasks: usize,
    #[serde(with = "humantime_serde")]
    pub scheduling_interval: Duration,
}
