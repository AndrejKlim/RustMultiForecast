// @generated automatically by Diesel CLI.

diesel::table! {
    forecast (forecast_id) {
        forecast_id -> Int4,
        forecast_json -> Text,
        source -> Text,
        duration -> Text,
        created -> Timestamp,
        user_id -> Int8,
    }
}

diesel::table! {
    location (location_id) {
        location_id -> Int4,
        lon -> Numeric,
        lat -> Numeric,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int8,
        location_id -> Int4,
    }
}

diesel::joinable!(forecast -> users (user_id));
diesel::joinable!(users -> location (location_id));

diesel::allow_tables_to_appear_in_same_query!(
    forecast,
    location,
    users,
);
