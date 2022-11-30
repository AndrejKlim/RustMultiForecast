-- Your SQL goes here
CREATE TABLE location
(
    location_id SERIAL PRIMARY KEY,
    lon         NUMERIC NOT NULL,
    lat         NUMERIC NOT NULL
);

CREATE TABLE users
(
    user_id     BIGINT PRIMARY KEY,
    location_id INT REFERENCES location(location_id) not null
);


