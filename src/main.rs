use std::sync::Mutex;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    required_input: String,
    maybe_other_input: Option<String>,
}

#[derive(Serialize)]
struct Output {
    required: String,
    maybe_other: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Animal {
    age: u32,
    animal: String,
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

// Rest (https://learn.microsoft.com/en-us/azure/architecture/best-practices/api-design)
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct Fruit {
    id: u32,
    name: String,
}

struct FruitList {
    fruits: Mutex<Vec<Fruit>>,
}

// Rest - get resource
#[get("/fruits/{id}")]
async fn get_fruit(fruit_id: web::Path<u32>, fruit_list: web::Data<FruitList>) -> HttpResponse {
    let id = fruit_id.into_inner();
    let maybe_fruit = fruit_list
        .fruits
        .lock()
        .unwrap()
        .iter()
        .find(|&fruit| fruit.id == id)
        .map(|fruit| fruit.clone());
    maybe_fruit
        .map(|fruit| HttpResponse::Ok().json(fruit))
        .unwrap_or(HttpResponse::NotFound().finish())
}

// Rest - update resource
#[put("/fruits/{id}")]
async fn update_fruit(
    fruit: web::Json<Fruit>,
    fruit_list: web::Data<FruitList>,
) -> Result<impl Responder> {
    let mut fruits = fruit_list.fruits.lock().unwrap();
    let maybe_fruit = fruits.iter_mut().find(|frt| frt.id == fruit.id);
    match maybe_fruit {
        Some(found_fruit) => (*found_fruit).name = fruit.into_inner().name,
        None => fruits.push(fruit.into_inner()),
    };
    Ok("".to_string())
}

// Rest - delete resource
#[delete("/fruits/{id}")]
async fn delete_fruit(fruit_id: web::Path<u32>) -> Result<impl Responder> {
    Ok("")
}

// Rest - list resources
#[get("/fruits")]
async fn get_fruits() -> Result<impl Responder> {
    Ok("")
}

// Rest - list resources
#[post("/fruits")]
async fn create_fruit() -> Result<impl Responder> {
    Ok("")
}
// ------------------------------------------------------------------------------------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let fruit_list = web::Data::new(FruitList {
        fruits: Mutex::new(vec![Fruit {
            id: 5,
            name: "pear".to_string(),
        }]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(fruit_list.clone())
            .service(hello)
            .service(echo)
            .service(api_get_my_animal_result_responder)
            .service(post_with_body_deserialized)
            .service(
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
                            .service(api_v2_get_hello_b_query_params)
                            .service(path_dynamic_segments)
                            .service(path_struct)
                            .service(path_struct_path_query)
                            .service(get_fruit)
                            .service(update_fruit),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
