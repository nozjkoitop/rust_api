use crate::schema::cars;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as JsonValue};

fn default_properties() -> JsonValue {
    JsonValue::Object(Map::new())
}

#[derive(Identifiable, Queryable, Serialize)]
#[diesel(table_name = cars)]
pub struct Car {
    pub id: i64,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub price: BigDecimal,
    pub created_at: NaiveDateTime,
    pub description: Option<String>,
    pub mileage: i32,
    pub engine: String,
    pub transmission: String,
    pub properties: JsonValue,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = cars)]
pub struct NewCar {
    pub make: String,
    pub model: String,
    pub year: i32,
    pub price: BigDecimal,
    pub description: Option<String>,
    pub mileage: i32,
    pub engine: String,
    pub transmission: String,
    #[serde(default = "default_properties")]
    #[diesel(sql_type = diesel::sql_types::Jsonb)]
    pub properties: JsonValue,
}
