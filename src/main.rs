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
use diesel::debug_query;
//use diesel::query_builder::AsChangeset;

mod db;

extern crate rocket_contrib;
use db::models::{Category, CategoryLink, Component, Page, Spell, SpellCategory, SpellComponent};
use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::response::content;

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


#[derive(Serialize)]
struct SpellContext {
    spell: Spell,
    categories: Vec<SpellCategory>,
    components: Vec<SpellComponent>,
}

#[get("/spell/<name>")]
fn get_single_spell(conn: db::Conn, name: String) -> Template {
    use db::schema::spell::dsl::spell;
    use db::schema::spell_category::dsl::spell_category;
    use db::schema::spell_component::dsl::spell_component;
    let item = spell
        .find(name)
        .get_result::<Spell>(&*conn)
        .expect("Failed to fetch spell");
    let cats = spell_category
        .filter(db::schema::spell_category::dsl::spell_id.eq(&item.name))
        .order(db::schema::spell_category::level)
        .load::<SpellCategory>(&*conn)
        .expect("Failed to fetch categories for spell");
    let cmps = spell_component
        .filter(db::schema::spell_component::dsl::spell_id.eq(&item.name))
        .order(db::schema::spell_component::component_id)
        .load::<SpellComponent>(&*conn)
        .expect("Failed to fetch components for spell");
    let ctx = SpellContext {
        spell: item,
        categories: cats,
        components: cmps,
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
    use db::schema::category_link::dsl::category_link;
    use db::schema::spell_category::dsl::spell_category;
    let item = category
        .find(name)
        .get_result::<Category>(&*conn)
        .expect("Failed to fetch category");
    let requirements = category_link
        .filter(db::schema::category_link::dsl::category_id.eq(&item.name))
        .order(db::schema::category_link::level)
        .load::<CategoryLink>(&*conn)
        .expect("Failed to fetch requirements for category");
    let required_by = category_link
        .filter(db::schema::category_link::dsl::required_id.eq(&item.name))
        .order(db::schema::category_link::level)
        .load::<CategoryLink>(&*conn)
        .expect("Failed to fetch requirements of category");
    let spells = spell_category
        .filter(db::schema::spell_category::dsl::category_id.eq(&item.name))
        .order(db::schema::spell_category::level)
        .load::<SpellCategory>(&*conn)
        .expect("Failed to fetch spells for category");
    let ctx = CategoryContext {
        category: item,
        requirements: requirements,
        required_by: required_by,
        spells: spells,
    };
    Template::render("category", ctx)
}

#[derive(Serialize)]
struct CategoryContext {
    category: Category,
    requirements: Vec<CategoryLink>,
    required_by: Vec<CategoryLink>,
    spells: Vec<SpellCategory>,
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
    // use db::schema::component_subset::dsl::component_subset;
    use db::schema::spell_component::dsl::spell_component;

    let thiscomponent = component
        .find(name)
        .get_result::<Component>(&*conn)
        .expect("Failed to fetch component");

    // Not sure what to do with this precisely, yet
    // let subsets = component_subset

    let spells = spell_component
        .filter(db::schema::spell_component::dsl::component_id.eq(&thiscomponent.name))
        .order(db::schema::spell_component::dsl::spell_id)
        .load::<SpellComponent>(&*conn)
        .expect("Failed to fetch spells for component");

    let ctx = ComponentContext {
        component: thiscomponent,
        // Other things, probably?
        spells: spells,
    };

    Template::render("component", ctx)
}

#[derive(Serialize)]
struct ComponentContext {
    component: Component,
    // Other things, probably?
    spells: Vec<SpellComponent>,
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

#[post("/page", data = "<page_form>")]
fn post_page(conn: db::Conn, page_form: Form<Page>) -> Result<Redirect, String> {
    use db::schema::page::dsl::page;
    let new_page = page_form.get();
    let insert_count = diesel::insert_into(page)
        .values(new_page)
        .on_conflict(db::schema::page::name)
        .do_update()
        .set(new_page)
        .execute(&*conn);
    match insert_count {
        Ok(_) => Ok(Redirect::to(&format!("/page/{}", new_page.name))),
        Err(e) => Err(format!("Failed: {}", e).to_string()),
    }
}

#[derive(FromForm)]
struct QueryAction {
    edit: Option<String>,
}
#[get("/page?<q>")]
fn new_page(conn: db::Conn, q: QueryAction) -> Template {
    Template::render("page_edit", "")
}
#[get("/page/<name>?<q>")]
fn edit_page(conn: db::Conn, name: String, q: QueryAction) -> Template {
    use db::schema::page::dsl::page;
    let mut ctx = HashMap::new();
    let thispage = page.find(name)
        .get_result::<Page>(&*conn)
        .expect("Failed to fetch page");
    ctx.insert("page", thispage);
    Template::render("page_edit", ctx)
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
                edit_page,
                post_page,
                new_page,
                get_components,
                get_single_component,
                get_categories,
                get_single_category,
                get_spells,
                get_single_spell,
                search_all,
            ],
        )
        .attach(Template::fairing())
        .launch();
    println!("Exiting!");
}
