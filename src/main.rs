use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use learn_actix_web::{
    graphql::{graphql_post, index_graphiql, EmployeeQuery, MergedQuery},
    rest::{delete_fruit, get_fruit, get_fruits, update_fruit, Fruit, FruitList},
    simple::{
        api_get_hello, api_get_hello_b, api_get_my_animal_result_responder, echo, hello,
        post_with_body_deserialized,
    },
    v1::{api_v1_get_hello, api_v1_get_hello_b},
    v2::{
        api_v2_get_hello, api_v2_get_hello_b, api_v2_get_hello_b_query_params,
        path_dynamic_segments, path_struct, path_struct_path_query,
    },
    web_socket::web_socket::index,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let fruit_list = web::Data::new(FruitList {
        fruits: Mutex::new(vec![Fruit {
            id: 5,
            name: "pear".to_string(),
        }]),
    });

    let schema = Schema::build(MergedQuery::default(), EmptyMutation, EmptySubscription).finish();

    println!("GraphiQL IDE: http://localhost:8080/graphql");

    HttpServer::new(move || {
        App::new()
            .app_data(fruit_list.clone())
            .app_data(web::Data::new(schema.clone()))
            .route("/ws/", web::get().to(index))
            .service(hello)
            .service(echo)
            .service(api_get_my_animal_result_responder)
            .service(post_with_body_deserialized)
            .service(index_graphiql)
            .service(graphql_post)
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
                            .service(update_fruit)
                            .service(delete_fruit)
                            .service(get_fruits),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
