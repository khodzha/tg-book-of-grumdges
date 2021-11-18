use std::env;

use diesel::Connection;
use diesel::PgConnection;

diesel_migrations::embed_migrations!("migrations");

pub fn run() {
    let connection = establish_connection();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Migrations failed");
}

pub fn establish_connection() -> PgConnection {
    loop {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        match PgConnection::establish(&database_url) {
            Ok(conn) => {
                return conn;
            }
            Err(e) => {
                log::error!(
                    "Error connecting to {} before running migrations, err = {:?}",
                    database_url,
                    e
                );
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
    }
}
