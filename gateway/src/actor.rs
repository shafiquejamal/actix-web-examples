use std::{collections::HashMap, num::NonZeroUsize, sync::mpsc::Sender};

use actix::prelude::*;
use log::warn;
use lru::LruCache;

use crate::graphql::Person;

const LRU_CACHE_SIZE: usize = 500;

pub struct GlobalActor {
    persons: LruCache<String, Sender<Vec<Person>>>,
    person: LruCache<String, Sender<Person>>,
}

impl GlobalActor {
    pub fn new() -> Self {
        Self {
            persons: LruCache::new(NonZeroUsize::new(LRU_CACHE_SIZE).unwrap()),
            // person: HashMap::new(),
            person: LruCache::new(NonZeroUsize::new(LRU_CACHE_SIZE).unwrap()),
        }
    }
}

impl Actor for GlobalActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]

pub enum GlobalActorMessage {
    AddPersonsMapping(String, Sender<Vec<Person>>),
    SendPersonsMessage(String, Vec<Person>),
    AddPersonMapping(String, Sender<Person>),
    SendPersonMessage(String, Person),
}

impl Handler<GlobalActorMessage> for GlobalActor {
    type Result = ();

    fn handle(&mut self, msg: GlobalActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            GlobalActorMessage::AddPersonsMapping(request_id, tx) => {
                self.persons.put(request_id, tx);
            }
            GlobalActorMessage::SendPersonsMessage(request_id, persons) => {
                self.persons.pop(&request_id).map(|tx| tx.send(persons));
            }
            GlobalActorMessage::AddPersonMapping(request_id, tx) => {
                self.person.put(request_id, tx);
            }
            GlobalActorMessage::SendPersonMessage(request_id, person) => {
                self.person
                    .pop(&request_id)
                    .map(|tx| match tx.send(person) {
                        Ok(()) => (),
                        Err(e) => warn!("Error sending {e:?}"),
                    });
            }
        };
    }
}
