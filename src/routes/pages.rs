extern crate diesel;

use diesel::prelude::*;

use rocket_contrib::Template;

use ::db::Conn as DBConn;
use ::db::schema as Schema;

use ::db::models::{Page};
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::http::uri::Segments;
use ::routes::auth::UserPass;

#[post("/page", data = "<page_form>")]
fn post_page(_user: UserPass<String>, conn: DBConn, page_form: Form<Page>) -> Result<Redirect, String> {
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
    action: Option<String>,
}

#[get("/page?<q>")]
fn new_page(_user: UserPass<String>, q: QueryAction) -> Result<Template, String> {
    match q.action.as_ref().map(String::as_ref) {
        Some("new") => Ok(Template::render("page_edit", "")),
        Some(_)     => Err(format!("Invalid action: {}", q.action.unwrap()).to_string()),
        None        => Err("No action".to_string()),
    }
    //Ok(Template::render("page_edit", ""))
}

#[get("/page/<names..>?<q>")]
fn edit_page(_user: UserPass<String>, conn: DBConn, names: Segments, q: QueryAction) -> Result<Template, String> {
    let name: String = names.collect::<Vec<&str>>().join("/");
    use ::db::schema::page::dsl::page;
    let mut ctx = HashMap::new();
    let thispage = page.find(name)
        .get_result::<Page>(&*conn)
        .expect("Failed to fetch page");
    ctx.insert("page", thispage);
    match q.action.as_ref().map(String::as_ref) {
        Some("edit") => Ok(Template::render("page_edit", ctx)),
        Some(_)     => Err(format!("Invalid action: {}", q.action.unwrap()).to_string()),
        None        => Err("No action".to_string()),
    }
}

#[derive(Serialize)]
struct PageContext {
    page: Page,
    user: Option<String>,
}
#[get("/page/<names..>")]
fn get_single_page(user: Option<UserPass<String>>, conn: DBConn, names: Segments) -> Template {
    let name: String = names.collect::<Vec<&str>>().join("/");
    use ::db::schema::page::dsl::page;
    let thispage = page.find(name)
        .get_result::<Page>(&*conn)
        .expect("Failed to fetch page");
    let ctx = PageContext{
        page: thispage,
        user: user.map(|u| u.user),
    };
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
