use std::fmt;

use actix_web::{body::EitherBody, http::StatusCode, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;

use crate::api_error::Error;

pub enum ApiResponse<T> {
    Ok(T),
    Err(Error),
}

impl<T> Responder for ApiResponse<T>
where
    T: Serialize + fmt::Debug,
{
    type Body = EitherBody<String>;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let response = match self {
            Self::Ok(r) => serde_json::to_value(&r).map_or_else(
                |_| {
                    tracing::error!("Cannot serialize response: {:?}", r);
                    HttpResponse::InternalServerError()
                        .message_body("Failed to serialize response".to_owned())
                },
                |v| {
                    let response = json!({
                      "status": "ok",
                      "payload": v
                    });

                    HttpResponse::Ok()
                        .content_type(mime::APPLICATION_JSON)
                        .message_body(serde_json::to_string(&response).unwrap())
                },
            ),
            Self::Err(e) => {
                let response = json!({
                  "status": "error",
                  "error": e
                });

                serde_json::to_string(&response)
                    .map_err(|err| err.into())
                    .and_then(|body| {
                        HttpResponse::build(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
                            .content_type(mime::APPLICATION_JSON)
                            .message_body(body)
                    })
            }
        };
        match response {
            Ok(res) => res.map_into_left_body(),
            Err(err) => HttpResponse::from_error(err).map_into_right_body(),
        }
    }
}

impl<T, E> From<Result<T, E>> for ApiResponse<T>
where
    E: Into<Error>,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(e) => ApiResponse::Ok(e),
            Err(e) => ApiResponse::Err(e.into()),
        }
    }
}
