use crate::actor::GlobalActorMessage;
use actix::Addr;
use actix_web::{get, post, web, HttpResponse};
use async_graphql::{http::GraphiQLSource, Object, SimpleObject};
use async_graphql::{Context, EmptyMutation, EmptySubscription, MergedObject, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::actor::GlobalActor;

#[derive(Serialize)]
enum Command {
    GetPerson,
    GetPersons,
}

#[derive(Serialize)]
struct ServiceRequest {
    request_id: String,
    command: Command,
}

impl ServiceRequest {
    pub fn get_person(request_id: String) -> Self {
        Self {
            request_id,
            command: Command::GetPerson,
        }
    }

    pub fn get_persons(request_id: String) -> Self {
        Self {
            request_id,
            command: Command::GetPersons,
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct Id {
    pub number: i32,
    pub department: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub id: Id,
}

#[derive(Default)]
pub struct EmployeeQuery {
    pub value: i32,
    pub person: Option<Person>,
    pub persons: Vec<Person>,
}

#[Object]
impl EmployeeQuery {
    async fn value(&self) -> i32 {
        self.value
    }

    async fn person<'ctx>(&self, ctx: &Context<'ctx>) -> Option<Person> {
        let (tx, rx): (Sender<Person>, Receiver<Person>) = mpsc::channel();
        let addr = ctx.data::<Addr<GlobalActor>>().unwrap();
        let request_id = Uuid::new_v4();
        addr.send(GlobalActorMessage::AddPersonMapping(
            request_id.clone().to_string(),
            tx,
        ))
        .await
        .unwrap();
        let producer = (*ctx.data::<FutureProducer>().unwrap()).clone();
        let payload = json!(ServiceRequest::get_person(request_id.to_string())).to_string();
        let topic = "from_router";
        // TODO: There is duplication here with code in the next method. Refactor to reduce duplication.
        producer
            .send(
                FutureRecord::to(topic)
                    .payload(&payload)
                    .key(&request_id.to_string()),
                Duration::from_secs(0),
            )
            .await
            .unwrap();
        let duration = Duration::new(2, 0);
        let start_time = Instant::now();
        loop {
            let now = Instant::now();
            if start_time + duration > now {
                let duration = start_time + duration - now;
                match rx.recv_timeout(duration) {
                    Ok(p) => return Some(p),
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            } else {
                break;
            }
        }
        None
    }

    async fn persons<'ctx>(&self, ctx: &Context<'ctx>) -> Option<Vec<Person>> {
        let (tx, rx): (Sender<Vec<Person>>, Receiver<Vec<Person>>) = mpsc::channel();
        let addr = ctx.data::<Addr<GlobalActor>>().unwrap();
        let request_id = Uuid::new_v4();
        addr.send(GlobalActorMessage::AddPersonsMapping(
            request_id.to_string(),
            tx,
        ))
        .await
        .unwrap();
        let producer = (*ctx.data::<FutureProducer>().unwrap()).clone();
        let payload = json!(ServiceRequest::get_persons(request_id.to_string())).to_string();
        let topic = "from_router";
        producer
            .send(
                FutureRecord::to(topic)
                    .payload(&payload)
                    .key(&request_id.to_string()),
                Duration::from_secs(0),
            )
            .await
            .unwrap();

        let duration = Duration::new(2, 0);
        let start_time = Instant::now();
        loop {
            let now = Instant::now();
            if start_time + duration > now {
                let duration = start_time + duration - now;
                match rx.recv_timeout(duration) {
                    Ok(ps) => return Some(ps),
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            } else {
                break;
            }
        }
        None
    }
}

#[derive(Default)]
pub struct PetQuery {
    pub animal_type: &'static str,
}

#[Object]
impl PetQuery {
    async fn get_animal_type(&self) -> &'static str {
        self.animal_type
    }
}

#[derive(MergedObject, Default)]
pub struct MergedQuery(EmployeeQuery, PetQuery);

// This is route to the IDE - note the 'i'
#[get("/graphiql")]
pub async fn index_graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8080/graphql")
                .finish(),
        )
}

type MySchema = Schema<MergedQuery, EmptyMutation, EmptySubscription>;

#[post("/graphql")]
pub async fn graphql_post(schema: web::Data<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
