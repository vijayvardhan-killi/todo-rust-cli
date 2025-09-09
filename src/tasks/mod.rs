use serde::{Deserialize, Serialize};
use uuid::Uuid;


pub mod add;
pub mod list;
pub mod delete;
pub mod done;
pub mod utils;


#[derive(Serialize, Deserialize , Debug)]
pub struct Task {
    pub id : String,
    pub description : String,
    pub done : bool,
}

impl Task {
    pub fn new(description : &str) -> Self {
        Task {
            id : Uuid::new_v4().to_string(),
            description : description.to_string(),
            done : false,
        }
    }
}



