extern crate dotenv;

use actix_multipart::Multipart;
use actix_svelte_surreal::{create_todo, delete_todo, update_todo};
use actix_web::{
    get, post,
    web::{self},
    App, Error, HttpResponse, HttpServer, Responder,
};

use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
#[derive(Serialize, Deserialize)]
struct TodoFields {
    text: Option<String>,
    id: Option<String>,
    done: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct TodoQuery {
    _method: Option<String>,
}

async fn parse_multipart_form(mut form: Multipart) -> Result<TodoFields, Error> {
    let mut fields = TodoFields {
        text: None,
        id: None,
        done: None,
    };

    while let Some(mut field) = form.try_next().await? {
        let mut buffer = Vec::new();
        while let Some(chunk) = field.try_next().await? {
            buffer.extend_from_slice(&chunk);
        }
        let name = field.name();
        let string_value = String::from_utf8(buffer).unwrap();
        match name {
            "id" => fields.id = Some(string_value),
            "text" => fields.text = Some(string_value),
            "done" => fields.done = Some(string_value == "true"),
            _ => (),
        }
    }

    Ok(fields)
}

#[post("/api/todos")]
async fn create_update_or_delete_todo(
    form: Multipart,
    query: web::Query<TodoQuery>,
    db: web::Data<Surreal<Client>>,
) -> Result<HttpResponse, Error> {
    let fields = parse_multipart_form(form).await?;
    match &query._method {
        Some(method) => match method.as_str() {
            "PATCH" => {
                dbg!(&fields.id);
                update_todo(
                    db.as_ref(),
                    &fields.id.expect("ID required"),
                    fields.text,
                    fields.done,
                )
                .await;
            }
            "DELETE" => {
                delete_todo(db.as_ref(), &fields.id.expect("ID required")).await;
            }
            _ => panic!("Unsupported method {}", method),
        },
        None => {
            let todo = create_todo(
                db.as_ref(),
                &fields.text.expect("text is required"),
                fields.done.unwrap_or(false),
            )
            .await;
            return Ok(HttpResponse::Ok().json(todo));
        }
    }
    Ok(HttpResponse::Ok().finish())
}

#[get("/api/todos/{id}")]
async fn get_todo(path: web::Path<String>, db: web::Data<Surreal<Client>>) -> impl Responder {
    let id = path.into_inner();
    web::Json(actix_svelte_surreal::get_todo(db.as_ref(), &id).await)
}

#[get("/api/todos")]
async fn get_todos(db: web::Data<Surreal<Client>>) -> impl Responder {
    web::Json(actix_svelte_surreal::get_todos(db.as_ref()).await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be a 16 bit int");
    let path = std::env::var("STATIC_FILE_PATH").expect("STATIC_FILE_PATH must be set");
    let static_files = String::from(path.strip_suffix("/").unwrap_or(&path));

    // Surreal DB
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await.unwrap();

    HttpServer::new(move || {
        let app = App::new()
            .service(create_update_or_delete_todo)
            .service(get_todo)
            .service(get_todos)
            .app_data(web::Data::new(db.clone()));

        if cfg!(not(debug_assertions)) {
            return app.service(
                actix_files::Files::new("/", static_files.clone())
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open(
                            vec![static_files.clone(), "index.html".to_string()].join("/"),
                        )
                        .expect("index file should exist"),
                    ),
            );
        }
        app
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
