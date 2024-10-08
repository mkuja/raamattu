// This module is for phone app.

use axum::extract::{Path, State};
use axum::Json;
use crate::application_state::ApplicationState;
use crate::db::query::{get_book_chapter_count, get_books, get_chapter};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
pub use crate::db::query::Book;

#[derive(Serialize)]
pub enum NumChapters {
    NumChapters(u16),
    Error(Value),
}

#[derive(Serialize)]
pub enum Chapter {
    Verses(Vec<Verse>),
    Error(Value),
}

#[derive(Serialize)]
pub struct Verse {
    pub num: u16,
    pub text: String
}

impl Book {
    pub async fn enumerate_books(State(state): State<ApplicationState>) -> Json<Vec<Book>> {
        let mut conn = state.pool.acquire().await.unwrap();
        Json::from(get_books(&mut conn).await.unwrap())
    }

    pub async fn num_chapters(
        State(state): State<ApplicationState>,
        Path(book): Path<String>
    ) -> Json<NumChapters> {
        let mut conn = state.pool.acquire().await.unwrap();
        let num_chapters = get_book_chapter_count(&mut conn, &book).await;
        if let Ok(num_chapters) = num_chapters {
            return Json(NumChapters::NumChapters(num_chapters));
        }
        Json(NumChapters::Error(json!({
            "code": 400,
            "explanation": "Invalid book name. /api/v1/enumerate_books gives list of books."
        })))
    }

    pub async fn chapter(
        State(state): State<ApplicationState>,
        Path(path): Path<(String, u16)>,
    ) -> Json<Chapter> {
        let (book, chapter) = path;
        let mut conn = state.pool.acquire().await.unwrap();
        let verses = get_chapter(&mut conn, &book, chapter).await;
        if let Ok(verses) = verses {
            return Json(Chapter::Verses(verses.iter()
                .map(|t| {Verse{num: t.0, text: t.1.clone()}})
                .collect()));
        }
        Json(Chapter::Error(json!({
            "code": 400,
            "explanation": "Invalid chapter definition. Get book names from /api/v1/enumerate_books and number of chapters for books from /api/v1/:book/num_chapters"
        })))
    }
}
