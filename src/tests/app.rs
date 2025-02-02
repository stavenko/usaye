use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    error::Error,
    test, App,
};

use crate::{api::configurator, cfg::Config};

pub async fn create_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let config = Config::read_from_file("./configs/test-config.toml").unwrap();

    test::init_service(
        App::new()
            .app_data(configurator::task_processor(config))
            .configure(configurator::app_routes_configurator),
    )
    .await
}
