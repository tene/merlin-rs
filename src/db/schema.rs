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
    spell_produces (spell_id, component_id) {
        spell_id -> Varchar,
        component_id -> Varchar,
        notes -> Text,
        quantity -> Nullable<Numeric>,
    }
}

table! {
    page (name) {
        name -> Varchar,
        text -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    category,
    component,
    page,
    spell,
    category_link,
    spell_category,
    spell_component,
    spell_produces,
    component_subset,
);
