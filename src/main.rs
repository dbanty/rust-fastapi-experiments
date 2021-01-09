use actix_service::{IntoServiceFactory, Service, ServiceFactory};
use actix_web::body::{Body, ResponseBody};
use actix_web::dev::{AppConfig, AppService, ServiceResponse};
use actix_web::test::{init_service, TestRequest};
use actix_web::App;
use color_eyre::Result;
use netlify_lambda_http::{handler, lambda::run, Request, Response};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
    Apiv2Schema, OpenApiExt,
};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut service = App::new()
        .wrap_api()
        .service(web::resource("/echo").route(web::post().to(echo_pet)))
        .with_json_spec_at("/openapi.json")
        .build()
        .into_factory()
        .new_service(AppConfig::default())
        .await
        .expect("Could not create service");
    run(handler(|request: Request, _context| async {
        let req = TestRequest::with_uri(&request.uri().to_string())
            .method(request.method().clone())
            .set_json(request.body())
            .to_request();
        let response: ServiceResponse = service.call(req).await.unwrap();
        let body = match response.response().body() {
            ResponseBody::Body(b) => b,
            ResponseBody::Other(b) => b,
        };
        if let Body::Bytes(bytes) = body {
            Ok(Response::new(bytes.to_vec()))
        } else {
            Ok(Response::new(vec![]))
        }
    }))
    .await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
struct Pet {
    name: String,
    id: Option<i64>,
}

#[api_v2_operation]
async fn echo_pet(body: Json<Pet>) -> Result<Json<Pet>, ()> {
    Ok(body)
}
