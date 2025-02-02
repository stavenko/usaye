use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    Responder, Route,
};

use crate::{
    api_response::ApiResponse, commands::add_task, providers::task_processor::TaskProcessor,
};

pub fn route() -> Route {
    web::post().to(endpoint)
}
pub async fn endpoint(
    input: Json<add_task::Input>,
    task_processor: web::Data<Arc<TaskProcessor>>,
) -> impl Responder {
    let result: ApiResponse<_> =
        add_task::command(input.into_inner(), (*task_processor.into_inner()).clone())
            .await
            .into();
    result
}
