
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Animal {
    pub age: u32,
    pub animal: String,
}

#[derive(Deserialize)]
pub struct Input {
    pub required_input: String,
    pub maybe_other_input: Option<String>,
}
