use std::env;
use std::ops::Neg;
use bigdecimal::BigDecimal;
use chrono::{Utc};
use diesel::{Connection, insert_into, PgConnection, RunQueryDsl};
use crate::models::{Forecast, Location, NewForecast, NewLocation, User};
use crate::schema::*;
use diesel::prelude::*;
use crate::enums::{Duration, Source};

pub fn get_weather(user_id: i64) -> Option<String> {
    let conn = &mut connection();

    let location = users::table
        .inner_join(location::table)
        .filter(users::user_id.eq(user_id))
        .select((location::location_id, location::lon, location::lat))
        .first::<Location>(conn);

    if let Ok(location) = location {
        let last_user_forecast = forecast::table
            .filter(forecast::user_id.eq(user_id))
            .order(forecast::created.desc())
            .first::<Forecast>(conn);

        let actual_forecast: Option<Forecast> =
            if let Ok(last_user_forecast) = last_user_forecast {
                let forecast_not_expired = !is_expired(&last_user_forecast);
                if forecast_not_expired {
                    Some(last_user_forecast)
                } else {
                    request_weather(user_id, conn, location)
                }
            } else {
                request_weather(user_id, conn, location)
            };

        Some(actual_forecast.unwrap().forecast_json)
    } else {
        Some("Пользователь не найден. Отправьте геолокацию для регистрации".to_string())
    }
}

fn request_weather(user_id: i64, conn: &mut PgConnection, location: Location) -> Option<Forecast> {
    let lat: String = location.lat.to_string();
    let lon: String = location.lon.to_string();
    let app_id = env::var("OPEN_WEATHER_APP_ID").expect("OPEN_WEATHER_APP_ID must be set");
    let params = [("lat", lat.as_str()), ("lon", lon.as_str()),
        ("units", "metric"), ("lang", "ru"), ("appid", &app_id)];
    let get_open_weather_url = "https://api.openweathermap.org/data/2.5/onecall";
    let client = reqwest::blocking::Client::new();
    let response = client.get(get_open_weather_url).query(&params).send();

    if let Ok(response) = response {
        if let Ok(forecast) = response.text() {
            let new_forecast = NewForecast::new(
                forecast.clone(),
                Source::OpenWeather.to_str(),
                Duration::Multi.to_string(),
                Utc::now().naive_utc(),
                user_id);
            if let Ok(forecast) = insert_into(forecast::table)
                .values(new_forecast)
                .get_result::<Forecast>(conn) {
                return Some(forecast);
            };
        }
    }
    None
}

pub fn save_location(user_id: i64, longitude: f32, latitude: f32) {
    let conn = &mut connection();

    let lat = BigDecimal::try_from(latitude).unwrap().round(1);
    let lon = BigDecimal::try_from(longitude).unwrap().round(1);
    let new_location = NewLocation { lat, lon };

    if let Ok(loc) = insert_into(location::table)
        .values(&new_location)
        .get_result::<Location>(conn) {
        let user_result = users::table
            .find(user_id)
            .first::<User>(conn);
        if let Ok(_) = user_result {
            let _ = diesel::update(users::table.find(user_id))
                .set(users::location_id.eq(loc.location_id))
                .execute(conn);
        } else {
            let _ = insert_into(users::table)
                .values(User { user_id, location_id: loc.location_id })
                .get_result::<User>(conn);
        }
    }
}


pub fn connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn is_expired(forecast: &Forecast) -> bool {
    let now = Utc::now().naive_utc();
    let to_compare = forecast.created;

    let duration = now.signed_duration_since(to_compare);

    if let Ok(duration_type) = Duration::from_str(&forecast.duration) {
        match duration_type {
            Duration::Weather => duration.num_hours() > 2,
            Duration::Nearest => duration.num_hours() > 12,
            Duration::LongTerm => duration.num_hours() > 24,
            Duration::Multi => duration.num_hours() > 24
        }
    } else {
        false
    }
}