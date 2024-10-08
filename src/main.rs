mod db;
mod serve_static;
mod templates;
mod search;
mod api;
mod application_state;
mod sitemap;



use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;
use tantivy::{Index, IndexReader};
use axum::{routing::get, Router, response::Html};
use sqlx::{sqlite::SqliteConnectOptions, FromRow, Error, SqlitePool};
use askama::Template;
use tower_http::follow_redirect::policy::PolicyExt;
use application_state::ApplicationState;
use crate::sitemap::generate_sitemap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = SqliteConnectOptions::new()
        .filename("./db/raamattu.db")
        .read_only(true);
    let pool = SqlitePool::connect_with(opts)
        .await
        .expect("could not get connection pool");

    // Prepare searches with Tantivy.
    let mut conn = pool.acquire().await?;
    let (index, reader) = search::build_index(&mut conn)
        .await
        .expect("could not build search index");

    generate_sitemap(&mut conn).await;
    drop(conn);

    let mut app_state = ApplicationState {
        pool,
        index,
        reader
    };

    let app = Router::new()
        .route("/", get(templates::frontpage::front_page))
        .route("/search", get(search::search_route))
        .route("/about", get(about))
        .route("/search-help", get(search_help))
        .route(
            "/books/:short_name",
            get(templates::chapter_number_page::chapter_numbers_page),
        )
        .route(
            "/books/:short_name/:chapter_num",
            get(templates::chapter_page::chapter_page),
        )
        .route("/api/v1/enumerate_books", get(api::Book::enumerate_books))
        .route("/api/v1/:book/num_chapters", get(get(api::Book::num_chapters)))
        .route("/api/v1/:book/:chapter", get(api::Book::chapter))
        .with_state(app_state)
        .nest_service("/static", serve_static::serve_static::serve_static());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Template)]
#[template(path="about.jinja")]
struct About{}
async fn about() -> Result<Html<String>, (StatusCode, String)> {
    Ok(Html(About{}.render().unwrap()))
}

#[derive(Template, FromRow)]
#[template(path="search_help.jinja")]
pub struct SearchHelp{
    pub bible_books: Vec<(String, String)>,
}

async fn search_help(State(state): State<ApplicationState>) -> Result<Html<String>, (axum::http::StatusCode, String)> {
    let mut conn = state.pool.acquire().await.unwrap();
    let sh = SearchHelp::new(&mut conn).await;
    Ok(Html(sh.render().unwrap()))
}

impl SearchHelp {
    pub async fn new(conn: &mut sqlx::SqliteConnection) -> SearchHelp {
        let rows: Vec<(String, String)> = sqlx::query_as("SELECT short_name as short, long_name as long FROM books")
            .fetch_all(conn)
            .await
            .unwrap();
        SearchHelp {
            bible_books: rows,
        }
    }
}
