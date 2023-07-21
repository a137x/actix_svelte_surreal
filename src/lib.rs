use self::models::Todo;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb::sql::thing;
use uuid::Uuid;
pub mod models;

pub async fn create_todo<'a>(
    db: &Surreal<Client>,
    text: &'a str,
    done: bool,
) -> Todo {
    let id = &Uuid::new_v4().to_string();

    // Create a new person with a random id
    let created: Option<Todo> = db
        .create("todo")
        .content(Todo {
            id: Thing::from(("todo", id.as_str())),
            text: text.to_string(),
            done: done,
        })
        .await
        .unwrap();
    // dbg!(created.clone());

    created.unwrap()
}

pub async fn update_todo<'a>(
    db: &Surreal<Client>,
    table_and_id: &'a str,
    text: Option<String>,
    done: Option<bool>,
) {
    let thing = thing(table_and_id).unwrap();
    dbg!(&thing);
    if let Some(txt) = text {
        db.query("UPDATE $thing SET text=$text;")
            .bind(("thing", &thing))
            .bind(("text", txt))
            .await
            .unwrap();
    }
    if let Some(dn) = done {
        db.query("UPDATE $thing SET done=$done;")
            .bind(("thing", &thing))
            .bind(("done", dn))
            .await
            .unwrap();
    }
}

pub async fn delete_todo<'a>(db: &Surreal<Client>, table_and_id: &'a str) {
    let thing = thing(table_and_id).unwrap();
    let _res: Option<Todo> = db.delete((thing.tb, thing.id)).await.unwrap();
}

pub async fn get_todo<'a>(db: &Surreal<Client>, uid: &'a str) -> Todo {
    let todo: Option<Todo> = db.select(("todo", uid)).await.unwrap();
    todo.unwrap()
}

pub async fn get_todos<'a>(
    db: &Surreal<Client>
) -> Vec<Todo> {
    let vec_todos: Vec<Todo> = db
        .query("SELECT * FROM type::table($table);")
        .bind(("table", "todo"))
        .await
        .unwrap()
        .take(0)
        .unwrap();
    // dbg!(&vec_todos);

    vec_todos
}
