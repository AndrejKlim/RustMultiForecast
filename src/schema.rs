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
    forecast_field_user_preferences (id) {
        id -> Int4,
        source -> Text,
        field -> Text,
        search_path -> Text,
        search_result_type -> Text,
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
    user_field_preferences (user_id, pref_id) {
        user_id -> Int8,
        pref_id -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int8,
        location_id -> Int4,
        last_command -> Nullable<Text>,
    }
}

diesel::joinable!(forecast -> users (user_id));
diesel::joinable!(user_field_preferences -> forecast_field_user_preferences (pref_id));
diesel::joinable!(user_field_preferences -> users (user_id));
diesel::joinable!(users -> location (location_id));

diesel::allow_tables_to_appear_in_same_query!(
    forecast,
    forecast_field_user_preferences,
    location,
    user_field_preferences,
    users,
);
