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
use db::models::{Category, Component, Page, Spell, SpellCategory};
use std::collections::HashMap;
use rocket_contrib::Template;

#[derive(Serialize)]
struct SpellContext {
    spell: Spell,
    categories: Vec<SpellCategory>,
}

#[get("/spell/<name>")]
fn get_single_spell(conn: db::Conn, name: String) -> Template {
    use db::schema::spell::dsl::spell;
    use db::schema::spell_category::dsl::spell_category;
    let item = spell
        .find(name)
        .get_result::<Spell>(&*conn)
        .expect("Failed to fetch spell");
    let cats = spell_category
        .filter(db::schema::spell_category::dsl::spell_id.eq(&item.name))
        .order(db::schema::spell_category::level)
        .load::<SpellCategory>(&*conn)
        .expect("Failed to fetch categories");
    let ctx = SpellContext {
        spell: item,
        categories: cats,
    };
    Template::render("spell", ctx)
}
#[get("/spells")]
fn get_spells(conn: db::Conn) -> Template {
    use db::schema::spell::dsl::spell;
    let mut ctx = HashMap::new();
    let items = spell
        .order(db::schema::spell::name)
        .load::<Spell>(&*conn)
        .expect("Failed to fetch spells");
    ctx.insert("spells", items);
    Template::render("spells", ctx)
}

#[get("/category/<name>")]
fn get_single_category(conn: db::Conn, name: String) -> Template {
    use db::schema::category::dsl::category;
    let mut ctx = HashMap::new();
    let item = category
        .find(name)
        .get_result::<Category>(&*conn)
        .expect("Failed to fetch category");
    ctx.insert("category", item);
    Template::render("category", ctx)
}
#[get("/categories")]
fn get_categories(conn: db::Conn) -> Template {
    use db::schema::category::dsl::category;
    let mut ctx = HashMap::new();
    let items = category
        .order(db::schema::category::name)
        .load::<Category>(&*conn)
        .expect("Failed to fetch categories");
    ctx.insert("categories", items);
    Template::render("categories", ctx)
}

#[get("/component/<name>")]
fn get_single_component(conn: db::Conn, name: String) -> Template {
    use db::schema::component::dsl::component;
    let mut ctx = HashMap::new();
    let thiscomponent = component
        .find(name)
        .get_result::<Component>(&*conn)
        .expect("Failed to fetch component");
    ctx.insert("component", thiscomponent);
    Template::render("component", ctx)
}
#[get("/components")]
fn get_components(conn: db::Conn) -> Template {
    use db::schema::component::dsl::component;
    let mut ctx = HashMap::new();
    let components = component
        .order(db::schema::component::name)
        .load::<Component>(&*conn)
        .expect("Failed to fetch components");
    ctx.insert("components", components);
    Template::render("components", ctx)
}

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
fn index() -> Template {
    //let mut ctx: HashMap<String, String> = HashMap::new();
    Template::render("index", "")
}

fn main() {
    //let conn = establish_connection();
    println!("Started!");
    rocket::ignite()
        .manage(db::init_pool())
        .mount(
            "/",
            routes![
                index,
                get_pages,
                get_single_page,
                get_components,
                get_single_component,
                get_categories,
                get_single_category,
                get_spells,
                get_single_spell
            ],
        )
        .attach(Template::fairing())
        .launch();
    println!("Exiting!");
}
