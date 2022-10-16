pub mod v1 {
    use actix_web::{get, HttpResponse, Responder};

    #[get("/api-v1-get")]
    async fn api_v1_get_hello() -> impl Responder {
        HttpResponse::Ok().body("'api_v1_get_hello'. scope: API, method: GET")
    }

    #[get("/api-v1-get-b")]
    async fn api_v1_get_hello_b() -> impl Responder {
        HttpResponse::Ok().body("'api_v1_get_hello_b'. scope: API, method: GET")
    }
}
