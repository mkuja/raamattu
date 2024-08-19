use std::time::Instant;

use askama::Template;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Html;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::db::query;

#[derive(Deserialize)]
pub struct SearchQuery {
    search: String,
}

#[derive(Deserialize, Serialize)]
pub struct Verse {
    book: String,
    chapter_nr: i32,
    verse_nr: i32,
    verse: String,
}

#[derive(Template)]
#[template(path="search.jinja")]
pub struct SearchContext {
    results: Vec<Verse>,
}

pub async fn search_route(
    State(pool): State<SqlitePool>,
    search_query: Query<SearchQuery>,
) -> Result<Html<String>, (StatusCode, String)> {
    let start = Instant::now();
    let mut conn = pool.acquire().await.expect("no database connection");
    let results;
    if search_query.search.is_empty() {
        results = query::query_by_text_search(&mut conn, "Jeesus elää")
            .await
            .unwrap();
    } else {
        results = query::query_by_text_search(&mut conn, &search_query.search)
            .await
            .unwrap();
    }
    let mut resp = vec![];
    for r in results {
        let verse =
            r.1.into_iter()
                .fold(String::new(), |a, b| a + &b.to_string() + " ");
        resp.push(Verse {
            book: r.0 .0,
            chapter_nr: r.0 .2,
            verse_nr: r.0 .3,
            verse,
        })
    }
    let stop = Instant::now();
    println!("{:?}", stop - start);
    Ok(Html(SearchContext {
        results: resp
    }.render().unwrap()))
}
