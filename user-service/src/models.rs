use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub enum Command {
    GetPerson,
    GetPersons,
}

#[derive(Deserialize)]
pub struct ServiceRequest {
    pub request_id: String,
    pub command: Command,
}

#[derive(Serialize)]
pub struct Id {
    pub number: i32,
    pub department: String,
}

#[derive(Serialize)]
pub struct Person {
    pub name: String,
    pub id: Id,
}

#[derive(Serialize)]
pub enum ResponseMessageDto {
    Person { person: Person },
    Persons { persons: Vec<Person> },
}

#[derive(Serialize)]
pub struct ResponseMessageDtoWrapper {
    pub response_message_dto: ResponseMessageDto,
    pub request_id: String,
    pub response_type: String,
}
