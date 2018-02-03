#![feature(nll)]
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate diesel;
extern crate diesel_full_text_search;
extern crate rocket;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate bigdecimal;

use diesel::prelude::*;
// use diesel::debug_query;
//use diesel::query_builder::AsChangeset;

mod db;
mod routes;

extern crate rocket_contrib;
use rocket_contrib::Template;

#[derive(FromForm)]
struct QuerySearch {
    text: String,
}

#[derive(Serialize, Queryable, Debug)]
struct SearchResult {
    id: String,
    headline: String,
}

#[derive(Serialize)]
struct SearchContext {
    query: String,
    pages: Vec<SearchResult>,
    spells: Vec<SearchResult>,
    components: Vec<SearchResult>,
    categories: Vec<SearchResult>,
}

#[get("/search?<search>")]
fn search_all(conn: db::Conn, search: QuerySearch) -> Template {
    use diesel_full_text_search::*;
    use db::schema::spell::dsl::spell;
    use db::schema::page::dsl::page;
    use db::schema::component::dsl::component;
    use db::schema::category::dsl::category;
    let query = plainto_tsquery(&search.text);

    let pagecontent = db::schema::page::name.concat(" ")
                      .concat(db::schema::page::text);

    let spellcontent = db::schema::spell::name.concat(" ")
                      .concat(db::schema::spell::description);

    let componentcontent = db::schema::component::name.concat(" ")
                      .concat(db::schema::component::description);

    let categorycontent = db::schema::category::name.concat(" ")
                      .concat(db::schema::category::description);

    let pages : Vec<SearchResult> = page
        .select((db::schema::page::name, ts_headline(&pagecontent, &query)))
        .filter(query.matches(to_tsvector(&pagecontent)))
        .load::<SearchResult>(&*conn)
        .expect("Failure searching pages");

    let spells : Vec<SearchResult> = spell
        .select((db::schema::spell::name, ts_headline(&spellcontent, &query)))
        .filter(query.matches(to_tsvector(&spellcontent)))
        .load::<SearchResult>(&*conn)
        .expect("Failure searching spells");

    let components : Vec<SearchResult> = component
        .select((db::schema::component::name, ts_headline(&componentcontent, &query)))
        .filter(query.matches(to_tsvector(&componentcontent)))
        .load::<SearchResult>(&*conn)
        .expect("Failure searching components");

    let categories : Vec<SearchResult> = category
        .select((db::schema::category::name, ts_headline(&categorycontent, &query)))
        .filter(query.matches(to_tsvector(&categorycontent)))
        .load::<SearchResult>(&*conn)
        .expect("Failure searching categories");

    let ctx = SearchContext{
        query: search.text,
        pages: pages,
        spells: spells,
        components: components,
        categories: categories,
    };
    Template::render("search", ctx)
}



fn main() {
    //let conn = establish_connection();
    println!("Started!");
    rocket::ignite()
        .manage(db::init_pool())
        .mount(
            "/",
            routes![
                routes::index,
                routes::pages::get_pages,
                routes::pages::get_single_page,
                routes::pages::edit_page,
                routes::pages::post_page,
                routes::pages::new_page,
                routes::components::get_components,
                routes::components::get_single_component,
                routes::spells::get_categories,
                routes::spells::get_single_category,
                routes::spells::get_spells,
                routes::spells::get_single_spell,
                search_all,
            ],
        )
        .attach(Template::fairing())
        .launch();
    println!("Exiting!");
}
