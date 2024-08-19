use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use sqlx::SqliteConnection;
use serde::Deserialize;
use axum_extra::extract::Query;

use crate::db::query;
use crate::SqlitePool;

#[derive(Template)]
#[template(path = "chapter_page.jinja")]
struct ChapterPageContext {
    book_long_name: String,
    book_short_name: String,
    chapter_nr: u16,
    chapters_in_book: u16,
    verses: Vec<(u16, String)>,
    hilights: Vec<u16>,
}

impl ChapterPageContext {
    async fn new(
        conn: &mut SqliteConnection,
        book_short_name: String,
        chapter_nr: u16,
    ) -> Result<Self, sqlx::Error> {
        let long_name = query::get_book_long_name(conn, &book_short_name)
            .await
            .unwrap();
        let verses = query::get_chapter(conn, &book_short_name, chapter_nr)
            .await
            .unwrap();
        let chapters_in_book = query::get_book_chapter_count(conn, &book_short_name)
            .await
            .unwrap();
        Ok(ChapterPageContext {
            book_long_name: long_name,
            book_short_name,
            chapter_nr,
            chapters_in_book,
            verses,
            hilights: vec![],
        })
    }

    async fn new_with_hilights(
        conn: &mut SqliteConnection,
        book_short_name: String,
        chapter_nr: u16,
        hilights: Vec<u16>,
    ) -> Result<Self, sqlx::Error> {
        let mut chp_ctx = Self::new(conn, book_short_name, chapter_nr).await?;
        chp_ctx.hilights = hilights;
        Ok(chp_ctx)
    }
}

#[derive(Deserialize)]
pub struct HilightQuery {
    #[serde(default)]
    pub hls: Vec<u16>,
}

pub async fn chapter_page(
    State(pool): State<SqlitePool>,
    Path(p): Path<(String, u16)>,
    Query(q): Query<HilightQuery>,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut conn = pool.acquire().await.expect("db connection error");
    let chapter = ChapterPageContext::new_with_hilights(&mut conn, p.0, p.1, q.hls).await.unwrap();
    Ok(Html(chapter.render().unwrap()))
}

