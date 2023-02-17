use diesel::{insert_into, delete, RunQueryDsl};
use crate::models::{NewUserFieldPreference};
use crate::schema::*;
use diesel::prelude::*;
use crate::enums::{Field};
use log::{debug};
use crate::service::service::connection;

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