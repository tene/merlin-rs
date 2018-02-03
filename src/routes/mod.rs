use rocket_contrib::Template;

pub mod components;
pub mod pages;
pub mod spells;
pub mod auth;

#[get("/")]
fn index() -> Template {
    //let mut ctx: HashMap<String, String> = HashMap::new();
    Template::render("index", "")
}
