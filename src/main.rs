#![feature(async_closure)]
mod db;
mod serve_static;
mod templates;

use axum::{routing::get, Router, response::Html};
use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool};
use askama::Template;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = SqliteConnectOptions::new()
        .filename("./db/raamattu.db")
        .read_only(true);
    let pool = SqlitePool::connect_with(opts)
        .await
        .expect("could not get connection pool");

    let app = Router::new()
        .route("/", get(templates::frontpage::front_page))
        .route("/search", get(templates::search::search_route))
        .route("/about", get(async || {Html(About{}.render().unwrap())}))
        .route(
            "/books/:short_name",
            get(templates::chapter_number_page::chapter_numbers_page),
        )
        .route(
            "/books/:short_name/:chapter_num",
            get(templates::chapter_page::chapter_page),
        )
        .with_state(pool)
        .nest_service("/static", serve_static::serve_static::serve_static());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Template)]
#[template(path="about.jinja")]
struct About{}
