pub mod simple {

    use actix_web::{get, post, web, HttpResponse, Responder, Result};
    use serde::Serialize;

    use crate::models::models::{Animal, Input};

    #[derive(Serialize)]
    struct Output {
        required: String,
        maybe_other: Option<String>,
    }

    #[get("/")]
    async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[post("/echo")]
    async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body("Input was:".to_owned() + &req_body)
    }

    #[post("/post-with-body-deserialized")]
    async fn post_with_body_deserialized(input: web::Json<Input>) -> Result<impl Responder> {
        let ip = input.into_inner();
        let output = Output {
            required: ip.required_input,
            maybe_other: ip.maybe_other_input,
        };
        Ok(web::Json(output))
    }

    #[get("/api-get")]
    async fn api_get_hello() -> impl Responder {
        HttpResponse::Ok().body("'api_get_hello'. scope: API, method: GET")
    }

    #[get("/api-get-my-animal-result-responder")]
    async fn api_get_my_animal_result_responder() -> Result<impl Responder> {
        Ok(web::Json(Animal {
            age: 5,
            animal: "dog".to_string(),
        }))
    }

    #[get("/api-get-b")]
    async fn api_get_hello_b() -> impl Responder {
        HttpResponse::Ok().body("'api_get_hello_b'. scope: API, method: GET")
    }
}
