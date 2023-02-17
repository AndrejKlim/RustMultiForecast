use diesel::{RunQueryDsl};
use crate::schema::*;
use diesel::prelude::*;
use log::{debug};
use crate::service::service::connection;

pub fn save_last_user_command(user_id: &i64, command: String) {
    let conn = &mut connection();

    debug!("Saving last command: {} for user {}", &command, &user_id);
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
    if let Some(opt) = commands.get(0) {
        match opt {
            Some(command) => command.to_string(),
            None => "None".to_string(),
        }
    } else {
        "None".to_string()
    }
}