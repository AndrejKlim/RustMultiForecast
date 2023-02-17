use diesel::{insert_into, delete, RunQueryDsl, debug_query};
use crate::models::{NewUserFieldPreference, User, UserNotificationTimeUpdate};
use crate::schema::*;
use diesel::prelude::*;
use regex::Regex;
use crate::service::service::connection;
use log::{debug};

pub fn set_notification_time(text: &str, user_id: &i64) -> Result<String, String>{
    let parse_result = parse_time(text);
    if let Err(err) = parse_result {
        return Result::Err(err);
    }

    let mut conn = &mut connection();
    let user = users::table
        .find(user_id)
        .first::<User>(conn);

    if let Ok(_) = user {
        debug!("updating notification time - {}", &text);
        let _ = diesel::update(users::table.find(user_id))
            .set(users::notification_time.eq(Some(text.to_string())))
            .execute(conn);
    }
    Ok("Время уведомления успешно установлено".to_string())

}

pub fn delete_notification(user_id: &i64) {
    let mut conn = &mut connection();
    let user = users::table
        .find(user_id)
        .first::<User>(conn);

    if let Some(_) = user.unwrap().notification_time {
        let _ = diesel::update(users::table.find(user_id))
            .set(UserNotificationTimeUpdate { notification_time: None})
            .execute(conn);
    }
}

fn parse_time(text: &str) -> Result<String, String>{
    let time_regex = Regex::new(r"^(0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$").unwrap();
    let notification_time_valid = time_regex.is_match(text);
    if notification_time_valid {
        Result::Ok("ok".to_string())
    } else {
        debug!("Wrong notification time - {}", text);
        Result::Err("Неправильный формат времени".to_string())
    }
}