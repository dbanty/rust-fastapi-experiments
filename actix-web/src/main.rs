use std::future::Future;
use std::pin::Pin;

use actix_service::{IntoServiceFactory, Service, ServiceFactory};
use actix_web::body::{Body, ResponseBody};
use actix_web::dev::{AppConfig, ServiceResponse};
use actix_web::test::TestRequest;
use actix_web::App;
use aws_lambda_events::encodings::Body as LambdaBody;
use netlify_lambda_http::{handler, lambda::run, Context, Handler, Request, Response};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
    Apiv2Schema, OpenApiExt,
};
use serde::{Deserialize, Serialize};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let actix_handler = ActixHandler {};

    run(handler(actix_handler)).await
}

struct ActixHandler;
type ActixHandlerFuture<Resp, Err> = Pin<Box<dyn Future<Output = Result<Resp, Err>> + 'static>>;

impl Handler for ActixHandler {
    type Error = Error;
    type Response = Response<LambdaBody>;
    type Fut = ActixHandlerFuture<Self::Response, Self::Error>;

    fn call(&mut self, request: Request, _context: Context) -> Self::Fut {
        let req = TestRequest::with_uri(&request.uri().to_string())
            .method(request.method().clone())
            .set_json(request.body())
            .to_request();
        let service_fut = App::new()
            .wrap_api()
            .service(web::resource("/actix/echo").route(web::post().to(echo_pet)))
            .with_json_spec_at("/actix/openapi.json")
            .build()
            .into_factory()
            .new_service(AppConfig::default());

        let fut = async {
            let mut service = service_fut.await.unwrap();
            let actix_response: ServiceResponse = service.call(req).await.unwrap();
            let mut response = Response::builder();
            for (k, v) in actix_response.headers().into_iter() {
                response = response.header(k, v);
            }

            let body = match actix_response.response().body() {
                ResponseBody::Body(b) => b,
                ResponseBody::Other(b) => b,
            };
            let lambda_response = if let Body::Bytes(bytes) = body {
                response
                    .body(LambdaBody::Text(String::from_utf8(bytes.to_vec())?))
                    .unwrap()
            } else {
                response.body(LambdaBody::Empty).unwrap()
            };
            Ok::<Self::Response, Self::Error>(lambda_response)
        };
        Box::pin(fut)
    }
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
