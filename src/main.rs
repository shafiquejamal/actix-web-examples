use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    required_input: String,
    maybe_other_input: Option<String>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Input was:".to_owned() + &req_body)
}

#[get("/api-get")]
async fn api_get_hello() -> impl Responder {
    HttpResponse::Ok().body("'api_get_hello'. scope: API, method: GET")
}

#[get("/api-get-b")]
async fn api_get_hello_b() -> impl Responder {
    HttpResponse::Ok().body("'api_get_hello_b'. scope: API, method: GET")
}

#[get("/api-v1-get")]
async fn api_v1_get_hello() -> impl Responder {
    HttpResponse::Ok().body("'api_v1_get_hello'. scope: API, method: GET")
}

#[get("/api-v1-get-b")]
async fn api_v1_get_hello_b() -> impl Responder {
    HttpResponse::Ok().body("'api_v1_get_hello_b'. scope: API, method: GET")
}

#[get("/api-v2-get")]
async fn api_v2_get_hello() -> impl Responder {
    HttpResponse::Ok().body("'api_v2_get_hello'. scope: API, method: GET")
}

#[get("/api-v2-get-b")]
async fn api_v2_get_hello_b() -> impl Responder {
    HttpResponse::Ok().body("'api_v2_get_hello_b'. scope: API, method: GET")
}

#[get("/api-v2-get-b-query-params")]
async fn api_v2_get_hello_b_query_params(query_params: web::Query<Input>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "'api_v2_get_hello_b_query_params'. scope: API, method: GET, \nrequired_input:{},\nmaybe_other_input (or default):{}", 
        query_params.required_input, query_params.maybe_other_input.as_deref().unwrap_or("default")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(echo).service(
            web::scope("/api")
                .service(api_get_hello)
                .service(api_get_hello_b)
                .service(
                    web::scope("/v1")
                        .service(api_v1_get_hello)
                        .service(api_v1_get_hello_b),
                )
                .service(
                    web::scope("/v2")
                        .service(api_v2_get_hello)
                        .service(api_v2_get_hello_b)
                        .service(api_v2_get_hello_b_query_params),
                ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
