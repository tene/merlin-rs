use diesel::prelude::*;

use rocket_contrib::Template;

use ::db::models::{Category, CategoryLink, Spell, SpellCategory, SpellComponent, SpellProduct};

use ::db::Conn as DBConn;
use ::db::schema as Schema;

use std::collections::HashMap;


#[get("/category/<name>")]
fn get_single_category(conn: DBConn, name: String) -> Template {
    use ::db::schema::category::dsl::category;
    use ::db::schema::category_link::dsl::category_link;
    use ::db::schema::spell_category::dsl::spell_category;
    let item = category
        .find(name)
        .get_result::<Category>(&*conn)
        .expect("Failed to fetch category");
    let requirements = category_link
        .filter(Schema::category_link::dsl::category_id.eq(&item.name))
        .order(Schema::category_link::level)
        .load::<CategoryLink>(&*conn)
        .expect("Failed to fetch requirements for category");
    let required_by = category_link
        .filter(Schema::category_link::dsl::required_id.eq(&item.name))
        .order(Schema::category_link::level)
        .load::<CategoryLink>(&*conn)
        .expect("Failed to fetch requirements of category");
    let spells = spell_category
        .filter(Schema::spell_category::dsl::category_id.eq(&item.name))
        .order(Schema::spell_category::level)
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
fn get_categories(conn: DBConn) -> Template {
    use ::db::schema::category::dsl::category;
    let mut ctx = HashMap::new();
    let items = category
        .order(Schema::category::name)
        .load::<Category>(&*conn)
        .expect("Failed to fetch categories");
    ctx.insert("categories", items);
    Template::render("categories", ctx)
}

#[derive(Serialize)]
struct SpellContext {
    spell: Spell,
    categories: Vec<SpellCategory>,
    components: Vec<SpellComponent>,
    products: Vec<SpellProduct>,
}

#[get("/spell/<name>")]
fn get_single_spell(conn: DBConn, name: String) -> Template {
    use ::db::schema::spell::dsl::spell;
    use ::db::schema::spell_category::dsl::spell_category;
    use ::db::schema::spell_component::dsl::spell_component;
    use ::db::schema::spell_product::dsl::spell_product;
    let item = spell
        .find(name)
        .get_result::<Spell>(&*conn)
        .expect("Failed to fetch spell");
    let cats = spell_category
        .filter(Schema::spell_category::dsl::spell_id.eq(&item.name))
        .order(Schema::spell_category::level)
        .load::<SpellCategory>(&*conn)
        .expect("Failed to fetch categories for spell");
    let cmps = spell_component
        .filter(Schema::spell_component::dsl::spell_id.eq(&item.name))
        .order(Schema::spell_component::component_id)
        .load::<SpellComponent>(&*conn)
        .expect("Failed to fetch components for spell");
    let products = spell_product
        .filter(Schema::spell_product::dsl::spell_id.eq(&item.name))
        .order(Schema::spell_product::component_id)
        .load::<SpellProduct>(&*conn)
        .expect("Failed to fetch products for spell");
    let ctx = SpellContext {
        spell: item,
        categories: cats,
        components: cmps,
        products: products,
    };
    Template::render("spell", ctx)
}
#[get("/spells")]
fn get_spells(conn: DBConn) -> Template {
    use ::db::schema::spell::dsl::spell;
    let mut ctx = HashMap::new();
    let items = spell
        .order(Schema::spell::name)
        .load::<Spell>(&*conn)
        .expect("Failed to fetch spells");
    ctx.insert("spells", items);
    Template::render("spells", ctx)
}
