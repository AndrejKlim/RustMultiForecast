use dotenvy::dotenv;
use crate::telegram::bot::process_updates;

pub mod models;
pub mod schema;
pub mod telegram;
pub mod service;
mod enums;

fn main() {
    dotenv().ok();
    process_updates();
}