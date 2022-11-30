-- Your SQL goes here
create table forecast (
    forecast_id serial primary key,
    forecast_json text not null,
    source text not null,
    duration text not null,
    created timestamp without time zone not null,
    user_id bigint not null references users(user_id)
);