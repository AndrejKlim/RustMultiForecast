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

// fn main() {
//
//     let connection = &mut establish_connection();
//     let results = location
//         .load::<Location>(connection)
//         .expect("Error loading posts");
//
//     println!("Displaying {} locations", results.len());
//     for l in results {
//         println!("{}", l.lon);
//         println!("-----------\n");
//         println!("{}", l.lat);
//     }
// }