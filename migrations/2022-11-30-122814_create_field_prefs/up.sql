-- Your SQL goes here
create table forecast_field_user_preferences
(
    id                 serial primary key,
    source             text not null,
    field              text not null,
    search_path        text not null,
    search_result_type text not null
);

create table user_field_preferences
(
    user_id bigint not null references users (user_id),
    pref_id int    not null references forecast_field_user_preferences (id),
    constraint user_id_pref_id_pk PRIMARY KEY (user_id, pref_id)
);

insert into forecast_field_user_preferences(source, field, search_path, search_result_type)
VALUES ('OpenWeather', 'Temperature', '$.current.temp', 'f64'),
       ('OpenWeather', 'WindSpeed', '$.current.wind_speed', 'f64'),
       ('OpenWeather', 'Pressure', '$.current.pressure', 'i64'),
       ('OpenWeather', 'Humidity', '$.current.humidity', 'i64');

alter table users add column last_command text;