#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};

use std::{io::ErrorKind, sync::Arc};
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;

use crate::db::{AffectedRows, DB};
use cors::*;

mod cors;
mod db;
mod error;
mod prelude;
mod utils;

#[post("/task/<title>")]
async fn add_task(title: String, db: &State<DB>) -> Result<serde_json::Value, std::io::Error> {
    let task = db
        .add_task(title)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create task."))?;

    log::info!("{}", task.to_string());

    Ok(task)
}

#[get("/task/<id>")]
async fn get_task(id: String, db: &State<DB>) -> Result<serde_json::Value, std::io::Error> {
    let task = db
        .get_task(id)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to fetch task."))?;

    Ok(task)
}

#[get("/tasks")]
async fn get_all_tasks(db: &State<DB>) -> Result<serde_json::Value, std::io::Error> {
    let tasks = db
        .get_all_tasks()
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to fetch all tasks."))?;

    Ok(tasks)
}

#[patch("/task/<id>")]
async fn toggle_task(id: String, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let affected_rows = db
        .toggle_task(id)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(Json(affected_rows))
}

#[delete("/task/<id>")]
async fn delete_task(id: String, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let affected_rows = db
        .delete_task(id)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to delete task."))?;

    Ok(Json(affected_rows))
}

#[launch]
async fn rocket() -> _ {
    let ds = Arc::new(Datastore::new("file://temp.db").await.unwrap());
    let sesh = Session::owner().with_db("my_db").with_ns("my_ns");

    let db = DB { ds, sesh };

    rocket::build()
        .mount(
            "/",
            routes![add_task, get_task, get_all_tasks, toggle_task, delete_task],
        )
        .attach(CORS)
        .manage(db)
}
