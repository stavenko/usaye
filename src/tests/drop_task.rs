use crate::tests::app::create_app;
use std::time::Duration;

use actix_web::test::{self, read_body};
use serde::Deserialize;
use serde_json::json;

use crate::{api_error::Error, commands::add_task::Task, providers::types::TaskView};

#[derive(Deserialize)]
pub struct Response<T, E> {
    status: String,
    payload: Option<T>,
    error: Option<E>,
}

#[actix_web::test]
async fn drop_running_task() {
    let app = create_app().await;
    let payload = json! {{
        "task_url" : "http://example.com",
        "delay":"1h"
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

    let req = test::TestRequest::post()
        .uri("/public/list-tasks")
        .append_header(("content-type", "application/json"))
        .to_request();

    tokio::time::sleep(Duration::from_millis(500)).await;

    let response = test::call_service(&app, req).await;
    let status = response.status();
    let text = read_body(response).await;
    assert_eq!(status, 200);
    let result = serde_json::from_slice::<Response<Vec<TaskView>, Error>>(&text).unwrap();
    let pl = result.payload.unwrap();
    let id = pl[0].id;

    let payload = json! {{
        "id":id
    }};

    let req = test::TestRequest::post()
        .uri("/public/drop-task")
        .append_header(("content-type", "application/json"))
        .set_payload(serde_json::to_string_pretty(&payload).unwrap())
        .to_request();

    let response = test::call_service(&app, req).await;
    let status = response.status();

    assert_eq!(status, 200);

    let req = test::TestRequest::post()
        .uri("/public/list-tasks")
        .append_header(("content-type", "application/json"))
        .to_request();

    tokio::time::sleep(Duration::from_millis(500)).await;

    let response = test::call_service(&app, req).await;
    let status = response.status();
    let text = read_body(response).await;
    assert_eq!(status, 200);
    let result = serde_json::from_slice::<Response<Vec<TaskView>, Error>>(&text).unwrap();
    let pl = result.payload.unwrap();

    assert_eq!(status, 200);

    assert_eq!(pl.len(), 1);
}
#[actix_web::test]
async fn drop_pending_task() {
    let app = create_app().await;
    for _ in 0..5 {
        let payload = json! {{
            "task_url" : "http://example.com",
            "delay":"1h"
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
    }

    let req = test::TestRequest::post()
        .uri("/public/list-tasks")
        .append_header(("content-type", "application/json"))
        .to_request();

    tokio::time::sleep(Duration::from_millis(500)).await;

    let response = test::call_service(&app, req).await;
    let status = response.status();
    let text = read_body(response).await;
    assert_eq!(status, 200);
    let result = serde_json::from_slice::<Response<Vec<TaskView>, Error>>(&text).unwrap();
    let pl = result.payload.unwrap();
    let id = pl[0].id;

    let payload = json! {{
        "id":id
    }};

    let req = test::TestRequest::post()
        .uri("/public/drop-task")
        .append_header(("content-type", "application/json"))
        .set_payload(serde_json::to_string_pretty(&payload).unwrap())
        .to_request();

    let response = test::call_service(&app, req).await;
    let status = response.status();

    assert_eq!(status, 200);

    let req = test::TestRequest::post()
        .uri("/public/list-tasks")
        .append_header(("content-type", "application/json"))
        .to_request();

    tokio::time::sleep(Duration::from_millis(500)).await;

    let response = test::call_service(&app, req).await;
    let status = response.status();
    let text = read_body(response).await;
    assert_eq!(status, 200);
    let result = serde_json::from_slice::<Response<Vec<TaskView>, Error>>(&text).unwrap();
    let pl = result.payload.unwrap();

    assert_eq!(status, 200);
    assert_eq!(pl.len(), 4);
}
