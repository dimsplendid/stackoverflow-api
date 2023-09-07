#[macro_use]
extern crate rocket;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use dotenvy::dotenv;
use std::env;

use sqlx::postgres::PgPoolOptions;

mod cors;
mod handlers;
mod models;

use cors::*;
use handlers::*;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();

    dotenv().expect(".env file should be in the root of the project");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url. 
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated. 
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool");

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.  
    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch questions");

    info!("********* Question Records *********");
    info!("{:?}", recs);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}