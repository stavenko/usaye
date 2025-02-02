use std::sync::Arc;

use actix_web::web;

use crate::{cfg::Config, providers::task_processor::TaskProcessor};

use super::endpoints::{add_task, drop_task, get_task_result, list_tasks};

pub fn task_processor(app_config: Config) -> Arc<TaskProcessor> {
    TaskProcessor::new(app_config.task_runner)
}

pub fn app_routes_configurator(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::scope("/public").configure(public_api));
}

pub fn public_api(cfg: &mut web::ServiceConfig) {
    cfg.route("/add-task", add_task::route())
        .route("/list-tasks", list_tasks::route())
        .route("/get-task-result", get_task_result::route())
        .route("/drop-task", drop_task::route());
}
