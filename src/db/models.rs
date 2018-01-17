use diesel::data_types::PgInterval;

#[derive(Queryable, Serialize)]
pub struct Category {
    pub name: String,
    pub abbr: String,
    pub description: String,
}

#[derive(Queryable, Serialize)]
pub struct Component {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub cost: f32,
    pub weight: f32,
    pub volume: f32,
}

#[derive(Queryable, Serialize)]
pub struct Page {
    pub name: String,
    pub text: String,
}

#[derive(Queryable, Serialize)]
pub struct Spell {
    pub name: String,
    pub description: String,
    pub range: f32,
    //pub casting_time: PgInterval,
    //pub duration: Option<PgInterval>,
}
