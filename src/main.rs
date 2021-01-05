#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

fn main() {
    use self::schema::notes::dsl::*;

    let connection = establish_connection();
    let results = notes
        .limit(5)
        .load::<models::Note>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for note in results {
        println!("{}", note.text);
        println!("----------\n");
        println!("{}", note.created_at);
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}