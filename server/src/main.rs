use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{Connection, Result};

mod models;

use crate::models::plant::Plant;

fn YESSSS() -> Result<()> {
    let conn = Connection::open("db/plants.db")?;
    /*conn.execute(
        "CREATE TABLE if not exists plants (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            details TEXT
        )",
        (), // empty list of parameters.
    )?;

    let me = Plant {
        id: 0,
        name: String::from("Apple"),
        details:String::from("An apple is a round, edible fruit ..."),
    };
    conn.execute(
        "INSERT INTO plants (name, details) VALUES (?1, ?2)",
        (&me.name, &me.details),
    )?;*/

    let mut stmt = conn.prepare("SELECT id, name, details FROM plants")?;
    let plant_iter = stmt.query_map([], |row| {
        Ok(Plant {
            id: row.get(0)?,
            name: row.get(1)?,
            details: row.get(2)?,
        })
    })?;

    for plant in plant_iter {
        println!("Found person {:?}", plant.unwrap());
    }

    Ok(())
}


#[get("/")]
async fn hello() -> impl Responder {
    let yes = YESSSS();
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
