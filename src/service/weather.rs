use std::env;
use chrono::{Utc};
use diesel::{insert_into, PgConnection, RunQueryDsl};
use crate::models::{Forecast, Location, NewForecast, ForecastFieldPreference};
use crate::schema::*;
use diesel::prelude::*;
use serde_json::Value;
use crate::enums::{Duration, Source, Field};
use jsonpath_rust::JsonPathFinder;
use log::{debug};
use crate::service::service::connection;

pub fn get_weather(user_id: i64) -> Option<String> {
    let conn = &mut connection();

    // Get users location
    let location = users::table
        .inner_join(location::table)
        .filter(users::user_id.eq(user_id))
        .select((location::location_id, location::lon, location::lat))
        .first::<Location>(conn);

    if let Ok(location) = location {
        // if location exists - check if actual forecast exists
        let last_user_forecast = forecast::table
            .filter(forecast::user_id.eq(user_id))
            .order(forecast::created.desc())
            .first::<Forecast>(conn);

        let actual_forecast: Option<Forecast> =
            if let Ok(last_user_forecast) = last_user_forecast {
                // if forecast exists and not expired return it
                let forecast_not_expired = !is_expired(&last_user_forecast);
                if forecast_not_expired {
                    debug!("Unexpired forecast found");
                    Some(last_user_forecast)
                } else {
                    // otherwise request new forecast
                    debug!("Requesting new forecast");
                    request_weather(user_id, conn, location)
                }
            } else {
                debug!("Requesting new forecast");
                request_weather(user_id, conn, location)
            };

        // get user preferences
        let user_forecast_field_preferences = user_field_preferences::table
            .inner_join(users::table)
            .inner_join(forecast_field_user_preferences::table)
            .filter(users::user_id.eq(user_id))
            .select((forecast_field_user_preferences::id, forecast_field_user_preferences::source,
                     forecast_field_user_preferences::field, forecast_field_user_preferences::search_path,
                     forecast_field_user_preferences::search_result_type))
            .load::<ForecastFieldPreference>(conn);

        // get all fields from forecast by preferences and build response
        let forecast = actual_forecast.as_ref().unwrap().forecast_json.as_str();
        let forecast_values: Vec<String> = user_forecast_field_preferences.unwrap().iter()
            .map(|ffp| cast_by_type(JsonPathFinder::from_str(&forecast, ffp.search_path.as_str())
                                        .unwrap().find().get(0).unwrap(), ffp))
            .map(|(field, value)| format_forecast_message((field, value)))
            .collect();

        let forecast_result = forecast_values.join("\n");

        Some(forecast_result)
    } else {
        debug!("User not found: {}", &user_id);
        Some("Пользователь не найден. Отправьте геолокацию для регистрации".to_string())
    }
}

fn is_expired(forecast: &Forecast) -> bool {
    let now = Utc::now().naive_utc();
    let to_compare = forecast.created;

    let duration = now.signed_duration_since(to_compare);

    if let Some(duration_type) = Duration::from_str(&forecast.duration) {
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

fn format_forecast_message((field_name, value): (&str, String)) -> String {
    let field_opt = Field::from_str(field_name);

    if let Some(field) = field_opt {
        format!("{}: {} {}", field.as_ru_str(), field.convert(value), field.units()).to_string()
    } else {
        value
    }
}

fn cast_by_type<'a>(found_value: &Value, ffp: &'a ForecastFieldPreference) -> (&'a str, String) {
    let value = match ffp.search_result_type.as_str() {
        "f64" => found_value.as_f64().unwrap().to_string(),
        "i64" => found_value.as_i64().unwrap().to_string(),
        _ => String::new()
    };

    (&ffp.field, value)
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
                Duration::Weather.to_string(),
                Utc::now().naive_utc(),
                user_id);
            if let Ok(forecast) = insert_into(forecast::table)
                .values(new_forecast)
                .get_result::<Forecast>(conn) {
                debug!("Successfully recieved new forecast for user {}", &user_id);
                return Some(forecast);
            };
        }
    }
    None
}
