use bigdecimal::BigDecimal;
use crate::schema::cars;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize)]
#[diesel(table_name = cars)]
pub struct Car {
    pub id: i64,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub price: BigDecimal,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = cars)]
pub struct NewCar {
    pub make: String,
    pub model: String,
    pub year: i32,
    pub price: BigDecimal,
}
