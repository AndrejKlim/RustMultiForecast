use diesel::prelude::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i64,
    pub location_id: i32,
}

#[derive(Queryable, Debug)]
pub struct Location {
    pub location_id: i32,
    pub lon: BigDecimal,
    pub lat: BigDecimal,
}

#[derive(Insertable)]
#[diesel(table_name = location)]
pub struct NewLocation {
    pub lon: BigDecimal,
    pub lat: BigDecimal,
}

#[derive(Queryable, Debug)]
pub struct Forecast {
    pub forecast_id: i32,
    pub forecast_json: String,
    pub source: String,
    pub duration: String,
    pub created: NaiveDateTime,
    pub user_id: i64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = forecast)]
pub struct NewForecast {
    pub forecast_json: String,
    pub source: String,
    pub duration: String,
    pub created: NaiveDateTime,
    pub user_id: i64,
}

impl NewForecast {
    pub fn new(forecast_json: String, source: String, duration: String, created: NaiveDateTime, user_id: i64) -> NewForecast {
        NewForecast { forecast_json, source, duration, created, user_id }
    }
}

#[derive(Queryable, Debug)]
pub struct ForecastFieldPreference {
    pub id: i32,
    pub source: String,
    pub field: String,
    pub search_path: String,
    pub search_result_type: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = forecast_field_user_preferences)]
pub struct NewForecastFieldPreference {
    pub source: String,
    pub field: String,
    pub search_path: String,
    pub search_result_type: String,
}

impl NewForecastFieldPreference {
    pub fn new(source: String, field: String, search_path: String, search_result_type: String) -> NewForecastFieldPreference {
        NewForecastFieldPreference { source, field, search_path, search_result_type }
    }
}