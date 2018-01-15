#![feature(nll)]
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
use std::collections::HashMap;
use rocket_contrib::Template;

#[get("/page/<name>")]
fn get_single_page(conn: db::Conn, name: String) -> Template {
    use db::schema::page::dsl::page;
    let mut ctx = HashMap::new();
    let thispage = page.find(name)
        .get_result::<Page>(&*conn)
        .expect("Failed to fetch page");
    ctx.insert("page", thispage);
    Template::render("page", ctx)
}
#[get("/pages")]
fn get_pages(conn: db::Conn) -> Template {
    use db::schema::page::dsl::page;
    let mut ctx = HashMap::new();
    let pages = page.order(db::schema::page::name)
        .load::<Page>(&*conn)
        .expect("Failed to fetch pages");
    ctx.insert("pages", pages);
    Template::render("pages", ctx)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    //let conn = establish_connection();
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![index, get_pages, get_single_page])
        .attach(Template::fairing())
        .launch();
}
