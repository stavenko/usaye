use std::sync::Arc;

use actix_web::{web, Responder, Route};

use crate::{
    api_response::ApiResponse, commands::list_tasks, providers::task_processor::TaskProcessor,
};

pub fn route() -> Route {
    web::post().to(endpoint)
}
pub async fn endpoint(task_processor: web::Data<Arc<TaskProcessor>>) -> impl Responder {
    let result: ApiResponse<_> = list_tasks::command((*task_processor.into_inner()).clone())
        .await
        .into();
    result
}
