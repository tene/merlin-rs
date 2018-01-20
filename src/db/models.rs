use db::schema::*;

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

/*
use diesel::data_types::PgInterval;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Interval(pub PgInterval);

impl Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Interval", 3)?;
        state.serialize_field("microseconds", &self.0.microseconds)?;
        state.serialize_field("days", &self.0.days)?;
        state.serialize_field("months", &self.0.months)?;
        state.end()
    }
}
*/

#[derive(Identifiable, Queryable, PartialEq, Serialize)]
#[table_name = "spell"]
#[primary_key(name)]
pub struct Spell {
    pub name: String,
    pub description: String,
    pub range: f32,
    //pub casting_time: PgInterval,
    //pub duration: Option<PgInterval>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize)]
#[table_name = "spell_category"]
#[belongs_to(Spell)]
#[primary_key(spell_id, category_id)]
pub struct SpellCategory {
    pub spell_id: String,
    pub category_id: String,
    pub level: i32,
}
