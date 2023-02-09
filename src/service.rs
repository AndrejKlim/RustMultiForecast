use std::env;
use bigdecimal::BigDecimal;
use chrono::{Utc};
use diesel::{Connection, insert_into, delete, PgConnection, RunQueryDsl};
use crate::models::{Forecast, Location, NewForecast, NewLocation, User, ForecastFieldPreference, NewUserFieldPreference};
use crate::schema::*;
use diesel::prelude::*;
use serde_json::Value;
use crate::enums::{Duration, Source, Field};
use jsonpath_rust::JsonPathFinder;
use log::{debug};

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
                    Some(last_user_forecast)
                } else {
                    // otherwise request new forecast
                    request_weather(user_id, conn, location)
                }
            } else {
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
        Some("Пользователь не найден. Отправьте геолокацию для регистрации".to_string())
    }
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
                .values(User { user_id, location_id: loc.location_id, last_command: None })
                .execute(conn);
        }
    }
}

pub fn save_forecast_preferences(response_text: &str, user_id_to_update: &i64) {
    debug!("Unparsed response text: {}", response_text);
    let split: Vec<String> = response_text.split(",").into_iter().map(|num| num.trim().to_string()).collect();

    let values = Field::values();

    let field_names_to_save: Vec<&Field> = split.iter()
        .filter_map(|str_num| str_num.parse::<usize>().ok())
        .filter_map(|num| values.get(num))
        .collect();
    debug!("Fields to save: {:?}", field_names_to_save);

    let conn = &mut connection();

    let field_names: Vec<String> = field_names_to_save.iter().map(|f| f.as_str().to_string()).collect();

    let ffup_result = forecast_field_user_preferences::table
        .filter(forecast_field_user_preferences::field.eq_any(field_names))
        .select(forecast_field_user_preferences::id)
        .load::<i32>(conn).ok();

    if let Some(ffup_ids) = ffup_result {
        if !ffup_ids.is_empty() {
            let _ = delete(user_field_preferences::table)
                .filter(user_field_preferences::user_id.eq(user_id_to_update))
                .execute(conn);

            let to_insert: Vec<NewUserFieldPreference> = ffup_ids.iter()
                .map(|ff_id| NewUserFieldPreference::new(*user_id_to_update, *ff_id))
                .collect();

            let _ = insert_into(user_field_preferences::table)
                .values(to_insert)
                .execute(conn);
        }
    }
}

pub fn connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn save_last_user_command(user_id: &i64, command: String) {
    let conn = &mut connection();

    let _ = diesel::update(users::table.find(user_id))
        .set(users::last_command.eq(Some(command)))
        .execute(conn);
}

pub fn get_last_user_command(user_id: &i64) -> String {
    let conn = &mut connection();

    let command_result = users::table
        .filter(users::user_id.eq(user_id))
        .select(users::last_command)
        .load::<Option<String>>(conn);

    let commands: Vec<Option<String>> = command_result.unwrap();
    let opt = commands.get(0).unwrap();
    match opt {
        Some(command) => command.to_string(),
        None => "None".to_string(),
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
