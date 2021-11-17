use std::env;

use diesel::Connection;
use diesel::SqliteConnection;

diesel_migrations::embed_migrations!("migrations");

pub fn run() {
    let connection = establish_connection();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Migrations failed");
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
