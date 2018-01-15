#[derive(Queryable, Serialize, Deserialize)]
pub struct Page {
    pub name: String,
    pub text: String,
}
/*
CREATE TABLE public.component
(
  name character varying(256) NOT NULL,
  description text NOT NULL,
  unit character varying(256) NOT NULL DEFAULT 'units'::character varying,
  cost real NOT NULL DEFAULT 0,
  weight real NOT NULL DEFAULT 0,
  volume real NOT NULL DEFAULT 0,
  CONSTRAINT component_pkey PRIMARY KEY (name)
)
*/

#[derive(Queryable)]
pub struct Component {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub cost: f32,
    pub weight: f32,
    pub volume: f32,
}
