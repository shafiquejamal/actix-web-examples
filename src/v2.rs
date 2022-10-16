pub mod v2 {
    use actix_web::{get, web, HttpResponse, Responder, Result};

    use crate::models::models::{Animal, Input};

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

    #[get("/dynamic_segments/{age}/{animal}")]
    async fn path_dynamic_segments(path: web::Path<(u32, String)>) -> Result<String> {
        let (age, animal) = path.into_inner();
        Ok(format!(
            "Dynamic: The animal '{animal}' is {age} years old."
        ))
    }

    #[get("/struct/{age}/{animal}")]
    async fn path_struct(animal: web::Path<Animal>) -> Result<String> {
        Ok(format!(
            "Struct: The animal '{}' is {} years old.",
            animal.animal, animal.age
        ))
    }

    #[get("/struct-path-query/{age}/{animal}")]
    async fn path_struct_path_query(
        animal: web::Path<Animal>,
        path: web::Query<Input>,
    ) -> Result<String> {
        Ok(format!(
        "Struct: The animal '{}' is {} years old.\nQuery params are required_input:{}, maybe_other_input:{}",
        animal.animal, animal.age, path.required_input, path.maybe_other_input.as_ref().unwrap_or(&"default".to_string())
    ))
    }
}
