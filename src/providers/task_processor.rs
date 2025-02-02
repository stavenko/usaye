use std::{collections::HashMap, sync::Arc};

use futures::{channel, lock::Mutex};
use tap::TapFallible;
use tracing::error;
use uuid::Uuid;

use crate::api_error::Error;

use super::{
    config::TaskRunnerConfig,
    types::{Task, TaskResult, TaskView},
};

pub struct TaskProcessor {
    config: TaskRunnerConfig,
    pending_tasks: Mutex<Vec<Task>>,
    current_tasks: Mutex<Vec<Task>>,
    completed_tasks: Mutex<Vec<Task>>,
    drop_signal: Mutex<HashMap<Uuid, channel::oneshot::Sender<()>>>,
    task_results: Arc<Mutex<HashMap<Uuid, Result<TaskResult, Error>>>>,
}

impl TaskProcessor {
    pub fn new(config: TaskRunnerConfig) -> Arc<Self> {
        let this = Arc::new(Self {
            config,
            pending_tasks: Mutex::new(Default::default()),
            current_tasks: Mutex::new(Default::default()),
            task_results: Arc::new(Mutex::new(Default::default())),
            completed_tasks: Mutex::new(Default::default()),
            drop_signal: Mutex::new(Default::default()),
        });
        tokio::spawn(Self::task_scheduler(this.clone()));
        this
    }
    pub async fn drop_task(self: Arc<Self>, id: Uuid) {
        {
            let mut guard = self.pending_tasks.lock().await;

            if let Some(ix) = guard.iter().position(|item| item.id == id) {
                guard.remove(ix);
            }
        }

        {
            let mut guard = self.current_tasks.lock().await;

            if let Some(ix) = guard.iter().position(|item| item.id == id) {
                guard.remove(ix);
            }

            if let Some(watch) = self.drop_signal.lock().await.remove(&id) {
                watch.send(()).tap_err(|_| error!("Failed to drop ")).ok();
            }
        }
    }

    async fn task_runner(task: Task) -> Result<TaskResult, Error> {
        tokio::time::sleep(task.delay).await;

        let client = reqwest::Client::new();
        let response = client
            .get(task.url)
            .send()
            .await
            .tap_err(|err| tracing::error!("Error {err}"))
            .map_err(|err| Error {
                code: "REQUEST_SEND_ERROR".to_string(),
                message: err.to_string(),
            })?;

        let status = response.status();
        let size = response.bytes().await.map(|b| b.len()).unwrap_or(0);
        Ok(TaskResult {
            read_size: size,
            status: status.as_u16(),
        })
    }

    async fn task_scheduler(self: Arc<Self>) {
        loop {
            let this = self.clone();

            let task = {
                let mut running = this.current_tasks.lock().await;
                if running.len() < self.config.max_tasks {
                    if let Some(task) = this.pending_tasks.lock().await.pop() {
                        running.push(task.clone());
                        Some(task)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            if let Some(task) = task {
                let results = self.task_results.clone();

                let (tx, rx) = channel::oneshot::channel();

                self.drop_signal.lock().await.insert(task.id, tx);

                let future = |t: Task| async {
                    let id = t.id;
                    let runner = Self::task_runner(t);
                    let result = tokio::select! {
                        _ = rx => {
                            Err(Error{ code: "TASK_ABORTED".to_string(), message: "Task has been aborted before it is completed or scheduled".to_string() })
                        }
                        result = runner => result,
                    };

                    (*results.lock_owned().await).insert(id, result);
                };
                let this = self.clone();
                tokio::spawn(async move {
                    future(task.clone()).await;
                    let mut current = this.current_tasks.lock().await;
                    if let Some(ix) = current.iter().position(|item| item.id == task.id) {
                        current.remove(ix);
                    }
                    this.completed_tasks.lock().await.push(task.clone());
                });
            }
            tokio::time::sleep(this.config.scheduling_interval).await;
        }
    }

    pub async fn get_task_result(&self, task_id: Uuid) -> Option<Result<TaskResult, Error>> {
        self.task_results.lock().await.get(&task_id).cloned()
    }

    pub async fn add_task(&self, task: Task) {
        let mut guard = self.pending_tasks.lock().await;
        guard.push(task);
    }

    pub async fn list_tasks(&self) -> Vec<TaskView> {
        let mut result = Vec::new();
        for t in self.pending_tasks.lock().await.iter() {
            result.push(TaskView {
                id: t.id,
                url: t.url.to_owned(),
                task_status: super::types::TaskStatus::Pending,
            });
        }

        for t in self.current_tasks.lock().await.iter() {
            result.push(TaskView {
                id: t.id,
                url: t.url.to_owned(),
                task_status: super::types::TaskStatus::Running,
            });
        }
        for t in self.completed_tasks.lock().await.iter() {
            result.push(TaskView {
                id: t.id,
                url: t.url.to_owned(),
                task_status: super::types::TaskStatus::Completed,
            });
        }

        result
    }
}
