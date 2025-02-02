use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    Responder, Route,
};

use crate::{
    api_response::ApiResponse, commands::get_task_result, providers::task_processor::TaskProcessor,
};

pub fn route() -> Route {
    web::post().to(endpoint)
}
pub async fn endpoint(
    input: Json<get_task_result::Input>,
    task_processor: web::Data<Arc<TaskProcessor>>,
) -> impl Responder {
    let result: ApiResponse<_> =
        get_task_result::command(input.into_inner(), (*task_processor.into_inner()).clone())
            .await
            .into();
    result
}
