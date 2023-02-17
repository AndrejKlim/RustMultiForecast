use dotenvy::dotenv;
use crate::telegram::bot::process_updates;
use log4rs;

pub mod models;
pub mod schema;
pub mod telegram;
pub mod service;
mod enums;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    dotenv().ok();

    process_updates();
}