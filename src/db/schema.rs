table! {
    category (name) {
        name -> Varchar,
        abbr -> Bpchar,
        description -> Varchar,
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

allow_tables_to_appear_in_same_query!(category, component, page, spell,);
