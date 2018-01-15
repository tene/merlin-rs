#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate diesel;
extern crate rocket;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;

mod db;

extern crate rocket_contrib;
use db::models::Page;
use rocket_contrib::Json;
#[get("/pages")]
fn get_pages(conn: db::Conn) -> QueryResult<Json<Vec<Page>>> {
    db::schema::page::table
        .load::<Page>(&*conn)
        .map(|pages| Json(pages))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    //let conn = establish_connection();
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![index, get_pages])
        .launch();
}
