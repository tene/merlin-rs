table! {
    category (name) {
        name -> Varchar,
        abbr -> Bpchar,
        description -> Varchar,
    }
}

table! {
    category_link (category_id, required_id) {
        category_id -> Varchar,
        required_id -> Varchar,
        level -> Int4,
    }
}

table! {
    component (name) {
        name -> Varchar,
        description -> Text,
        unit -> Varchar,
        cost -> Float4,
        weight -> Float4,
        volume -> Float4,
    }
}

table! {
    component_subset (component_id, subset_of) {
        component_id -> Varchar,
        subset_of -> Varchar,
    }
}

table! {
    page (name) {
        name -> Varchar,
        text -> Text,
    }
}

table! {
    spell (name) {
        name -> Varchar,
        description -> Text,
        range -> Float4,
        casting_time -> Interval,
        duration -> Nullable<Interval>,
    }
}

table! {
    spell_category (spell_id, category_id) {
        spell_id -> Varchar,
        category_id -> Varchar,
        level -> Int4,
    }
}

table! {
    spell_component (spell_id, component_id) {
        spell_id -> Varchar,
        component_id -> Varchar,
        notes -> Text,
        quantity -> Nullable<Numeric>,
    }
}

table! {
    spell_product (spell_id, component_id) {
        spell_id -> Varchar,
        component_id -> Varchar,
        notes -> Text,
        quantity -> Nullable<Numeric>,
    }
}

joinable!(spell_category -> category (category_id));
joinable!(spell_category -> spell (spell_id));
joinable!(spell_component -> component (component_id));
joinable!(spell_component -> spell (spell_id));
joinable!(spell_product -> component (component_id));
joinable!(spell_product -> spell (spell_id));

allow_tables_to_appear_in_same_query!(
    category,
    category_link,
    component,
    component_subset,
    page,
    spell,
    spell_category,
    spell_component,
    spell_product,
);
