extern crate diesel;

use diesel::prelude::*;

use rocket_contrib::Template;

use ::db::Conn as DBConn;
use ::db::schema as Schema;

use ::db::models::{Page};
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket::request::Form;

#[post("/page", data = "<page_form>")]
fn post_page(conn: DBConn, page_form: Form<Page>) -> Result<Redirect, String> {
    use ::db::schema::page::dsl::page;
    let new_page = page_form.get();
    let insert_count = diesel::insert_into(page)
        .values(new_page)
        .on_conflict(Schema::page::name)
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
fn new_page(conn: DBConn, q: QueryAction) -> Template {
    Template::render("page_edit", "")
}

#[get("/page/<name>?<q>")]
fn edit_page(conn: DBConn, name: String, q: QueryAction) -> Template {
    use ::db::schema::page::dsl::page;
    let mut ctx = HashMap::new();
    let thispage = page.find(name)
        .get_result::<Page>(&*conn)
        .expect("Failed to fetch page");
    ctx.insert("page", thispage);
    Template::render("page_edit", ctx)
}

#[get("/page/<name>")]
fn get_single_page(conn: DBConn, name: String) -> Template {
    use ::db::schema::page::dsl::page;
    let mut ctx = HashMap::new();
    let thispage = page.find(name)
        .get_result::<Page>(&*conn)
        .expect("Failed to fetch page");
    ctx.insert("page", thispage);
    Template::render("page", ctx)
}

#[get("/pages")]
fn get_pages(conn: DBConn) -> Template {
    use ::db::schema::page::dsl::page;
    let mut ctx = HashMap::new();
    let pages = page.order(Schema::page::name)
        .load::<Page>(&*conn)
        .expect("Failed to fetch pages");
    ctx.insert("pages", pages);
    Template::render("pages", ctx)
}
