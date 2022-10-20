use std::sync::Mutex;

use actix::prelude::*;
use actix_web::{web, App, HttpServer};
use actix_web_static_files::ResourceFiles;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use gateway::{
    actor::GlobalActor,
    graphql::{graphql_post, index_graphiql, MergedQuery},
    kafka_consumer::IngestConsumer,
    kafka_producer::create_kafka_producer,
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
use std::env;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

const DEFAULT_BROKERS: &str = "localhost:29092";
const DEFAULT_CONSUMER_GROUP_ID: &str = "1";
const DEFAULT_LISTEN_TOPIC: &str = "from_service";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let global_actor_address = GlobalActor::new().start();

    let fruit_list = web::Data::new(FruitList {
        fruits: Mutex::new(vec![Fruit {
            id: 5,
            name: "pear".to_string(),
        }]),
    });

    // take brokers and topics to listen to, from arg, or environment, or default
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let brokers = args
        .pop()
        .or(env::var("USER_SERVICE_BROKERS").ok())
        .unwrap_or(DEFAULT_BROKERS.to_string());
    let group_id = args
        .pop()
        .or(env::var("USER_SERVICE_CONSUMER_GROUP_ID").ok())
        .unwrap_or(DEFAULT_CONSUMER_GROUP_ID.to_string());
    let listen_topics = args
        .pop()
        .or(env::var("USER_SERVICE_LISTEN_TOPICS").ok())
        .map(|l_ts| {
            l_ts.split(",")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
        })
        .unwrap_or(vec![DEFAULT_LISTEN_TOPIC.to_string()]);

    let producer = create_kafka_producer(&brokers).expect("Could not create Kafka producer");
    let schema = Schema::build(MergedQuery::default(), EmptyMutation, EmptySubscription)
        .data(global_actor_address.clone())
        .data(producer)
        .finish();

    println!("GraphiQL IDE: http://localhost:8080/graphql");

    let ingest_consumer =
        IngestConsumer::new(&brokers, &group_id, listen_topics, global_actor_address)
            .expect("failed to make ingest consumer");

    actix_rt::spawn(async move { ingest_consumer.run().await });

    HttpServer::new(move || {
        let generated = generate(); // For serving the React App
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
            .service(ResourceFiles::new("/", generated)) // Serves the React App
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
