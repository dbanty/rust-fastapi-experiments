#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

use rocket::Rocket;
use rocket_contrib::json::Json;
use rocket_lamb::RocketExt;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct Pet {
    name: String,
    id: Option<i64>,
}

#[openapi]
#[post("/echo", format = "json", data = "<pet>")]
fn echo(pet: Json<Pet>) -> Json<Pet> {
    pet
}

fn build_app() -> Rocket {
    rocket::ignite()
        .mount("/rocket", routes_with_openapi![echo])
        .mount(
            "/rocket/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}

fn main() {
    build_app().lambda().launch();
}

#[cfg(test)]
mod tests {
    use super::build_app;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    #[test]
    fn echo() {
        let client = Client::new(build_app()).expect("Could not build app");
        let req = client
            .post("/rocket/echo")
            .header(ContentType::JSON)
            .body(r#"{"name": "Bob"}"#);
        let mut resp = req.dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(
            resp.body_string(),
            Some(r#"{"name":"Bob","id":null}"#.to_string())
        );
    }
}
