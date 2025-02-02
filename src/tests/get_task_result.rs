use crate::{providers::types::TaskResult, tests::app::create_app};
use std::time::Duration;

use actix_web::test::{self, read_body};
use serde::Deserialize;
use serde_json::json;

use crate::{api_error::Error, commands::add_task::Task};

#[derive(Deserialize)]
pub struct Response<T, E> {
    status: String,
    payload: Option<T>,
    error: Option<E>,
}

#[actix_web::test]
async fn get_task_result() {
    let app = create_app().await;
    let payload = json! {{
        "task_url" : "https://www.youtube.com/watch?v=dQw4w9WgXcQ&ab_channel=RickAstley",
    }};
    let req = test::TestRequest::post()
        .uri("/public/add-task")
        .append_header(("content-type", "application/json"))
        .set_payload(serde_json::to_string_pretty(&payload).unwrap())
        .to_request();

    let response = test::call_service(&app, req).await;
    let status = response.status();
    let text = read_body(response).await;

    assert_eq!(status, 200);
    let result = serde_json::from_slice::<Response<Task, Error>>(&text).unwrap();

    assert_eq!(result.status, "ok");
    assert!(result.payload.is_some());
    assert!(result.error.is_none());

    let payload = json! {{
        "id" : result.payload.unwrap().id,
    }};

    let req = test::TestRequest::post()
        .uri("/public/get-task-result")
        .append_header(("content-type", "application/json"))
        .set_payload(serde_json::to_string_pretty(&payload).unwrap())
        .to_request();

    tokio::time::sleep(Duration::from_millis(5000)).await;
    let response = test::call_service(&app, req).await;
    let status = response.status();
    let text = read_body(response).await;

    println!("text {text:?}");
    assert_eq!(status, 200);
    let result =
        serde_json::from_slice::<Response<Option<Result<TaskResult, Error>>, Error>>(&text)
            .unwrap();

    assert!(result.payload.is_some());
}
