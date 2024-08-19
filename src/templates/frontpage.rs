
use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use sqlx::SqliteConnection;

use crate::SqlitePool;
use crate::db::query;


#[derive(Template)]
#[template(path="front_page.jinja")]
struct FrontPageContext {
    books: Vec<query::Book>,
}

impl FrontPageContext {
    async fn new(conn: &mut SqliteConnection) -> Result<Self, sqlx::Error> {
        let books = query::get_books(conn).await?;
        Ok(Self {
            books
        })
    }
}

pub async fn front_page(State(pool): State<SqlitePool>)
-> Result<Html<String>, (StatusCode, String)> {
    let mut conn = pool.acquire().await.expect("db connection error");
    let books = FrontPageContext::new(&mut conn).await.unwrap();
    Ok(Html(books.render().unwrap()))
}

