use crate::schema::images;
use crate::models::Car;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable, Associations, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[diesel(table_name = images)]
#[diesel(belongs_to(Car))]
pub struct Image {
    pub id: i64,
    pub car_id: i64,
    pub url: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = images)]
pub struct NewImage {
    pub car_id: i64,
    pub url: String,
}