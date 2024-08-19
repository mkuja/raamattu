use askama::Template;
use axum::extract::{
    State,
    Path
};
use axum::http::StatusCode;
use axum::response::Html;
use sqlx::SqliteConnection;

use crate::SqlitePool;
use crate::db::query;

#[derive(Template)]
#[template(path="chapters_page.jinja")]
struct ChapterNumberPageContext {
    book_long_name: String,
    book_short_name: String,
    num_chapters: u16,
}

impl ChapterNumberPageContext {
    async fn new(
        conn: &mut SqliteConnection,
        book_short_name: String,
        book_long_name: String,
    ) -> Result<Self, sqlx::Error> {
        let chapter_count = query::get_book_chapter_count(conn, &book_short_name).await?;
        Ok(Self {
            book_short_name: book_short_name.to_string(),
            book_long_name: book_long_name.to_string(),
            num_chapters: chapter_count,
        })
    }
}

pub async fn chapter_numbers_page(State(pool): State<SqlitePool>, Path(p): Path<String>)
-> Result<Html<String>, (StatusCode, String)> {
    let mut conn = pool.acquire().await.expect("db connection error");
    let long_name = query::get_book_long_name(&mut conn, &p).await.unwrap();
    let ch_nums = ChapterNumberPageContext::new(&mut conn, p.to_string(), long_name).await.unwrap();
    Ok(Html(ch_nums.render().unwrap()))
}

