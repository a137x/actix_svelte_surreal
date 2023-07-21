use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: Thing,
    pub text: String,
    pub done: bool,
}
