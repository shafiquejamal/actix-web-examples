use actix_web::{get, post, web, HttpResponse};
use async_graphql::{http::GraphiQLSource, Object, SimpleObject};
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[derive(SimpleObject)]
pub struct Id {
    pub number: i32,
    pub department: &'static str,
}

#[derive(SimpleObject)]
pub struct Person {
    pub name: &'static str,
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
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn value(&self) -> i32 {
        self.value
    }

    async fn person(&self) -> Person {
        Person {
            name: "Alice",
            id: Id {
                number: 1,
                department: "Executive",
            },
        }
    }

    async fn persons(&self) -> Vec<Person> {
        let alice = Person {
            name: "Alice",
            id: Id {
                number: 1,
                department: "Executive",
            },
        };
        let bob = Person {
            name: "Bob",
            id: Id {
                number: 2,
                department: "Finance",
            },
        };
        let charlie = Person {
            name: "Charlie",
            id: Id {
                number: 3,
                department: "Operations",
            },
        };
        vec![alice, bob, charlie]
    }
}

#[derive(Default)]
pub struct PetQuery {
    pub aminal_type: &'static str,
}

#[Object]
impl PetQuery {
    async fn get_animal_type(&self) -> &'static str {
        self.aminal_type
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
