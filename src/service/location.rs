use crate::service::service::connection;
use crate::schema::*;
use diesel::prelude::*;
use bigdecimal::BigDecimal;
use diesel::{insert_into, RunQueryDsl};
use crate::models::{Location, NewLocation, User};

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