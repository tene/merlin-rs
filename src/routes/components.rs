use diesel::prelude::*;

use rocket_contrib::Template;

use ::db::models::{Component, ComponentSubset, SpellComponent, SpellProduct};

use ::db::Conn as DBConn;
use ::db::schema as Schema;

use std::collections::HashMap;

#[get("/component/<name>")]
fn get_single_component(conn: DBConn, name: String) -> Template {
    use ::db::schema::component::dsl::component;
    use ::db::schema::component_subset::dsl::component_subset;
    use ::db::schema::spell_component::dsl::spell_component;
    use ::db::schema::spell_product::dsl::spell_product;

    let thiscomponent = component
        .find(name)
        .get_result::<Component>(&*conn)
        .expect("Failed to fetch component");

    let superset_of = component_subset
        .filter(Schema::component_subset::dsl::subset_of.eq(&thiscomponent.name))
        .order(Schema::component_subset::component_id)
        .load::<ComponentSubset>(&*conn)
        .expect("Failed to fetch supersets for component");

    let subset_of = component_subset
        .filter(Schema::component_subset::dsl::component_id.eq(&thiscomponent.name))
        .order(Schema::component_subset::component_id)
        .load::<ComponentSubset>(&*conn)
        .expect("Failed to fetch subsets for component");

    let spells = spell_component
        .filter(Schema::spell_component::dsl::component_id.eq(&thiscomponent.name))
        .order(Schema::spell_component::dsl::spell_id)
        .load::<SpellComponent>(&*conn)
        .expect("Failed to fetch spells for component");

    let producers = spell_product
        .filter(Schema::spell_product::dsl::component_id.eq(&thiscomponent.name))
        .order(Schema::spell_product::spell_id)
        .load::<SpellProduct>(&*conn)
        .expect("Failed to fetch producers for spell");

    let ctx = ComponentContext {
        component: thiscomponent,
        supersets: superset_of,
        subsets: subset_of,
        spells: spells,
        producers: producers,
    };

    Template::render("component", ctx)
}

#[derive(Serialize)]
struct ComponentContext {
    component: Component,
    supersets: Vec<ComponentSubset>,
    subsets: Vec<ComponentSubset>,
    spells: Vec<SpellComponent>,
    producers: Vec<SpellProduct>,
}

#[get("/components")]
fn get_components(conn: DBConn) -> Template {
    use ::db::schema::component::dsl::component;
    let mut ctx = HashMap::new();
    let components = component
        .order(Schema::component::name)
        .load::<Component>(&*conn)
        .expect("Failed to fetch components");
    ctx.insert("components", components);
    Template::render("components", ctx)
}
