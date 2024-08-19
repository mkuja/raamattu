use std::iter::zip;

use crate::db::query_types::Word;
use sqlx::{SqliteConnection,
    QueryBuilder,
    query_as,
    FromRow,
};
use itertools::Itertools;


// {{{ Free text search
/// Search by words in the query_str and return verses where all of the words appear.
/// Incomplete words are fine, as long as start of the word matches, and the search
/// term is longer than 2 characters.
///
/// The Ok return value is:
///     ((short_book_name, book_num, chaper_num, verse_num), Vec-of-words-in-verse).
pub async fn query_by_text_search(
    conn: &mut SqliteConnection,
    query_str: &str,
) -> Result<Vec<((String, i32, i32, i32), Vec<Word>)>, sqlx::Error> {
    let words: Vec<_> = query_str.split(" ").collect();
    let initial_qs = "
    SELECT * FROM words WHERE (book_number, chapter_number, verse_number) IN (SELECT book_number, chapter_number, verse_number FROM words WHERE word LIKE CONCAT(";
    let mut qb: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(initial_qs);
    qb.push_bind(words[0].to_string());
    qb.push(", '%')) ");
    for i in 1..words.len() {
        if words[i].len() < 3 { continue; }
        qb.push("AND (book_number, chapter_number, verse_number) IN (SELECT book_number, chapter_number, verse_number FROM words WHERE word LIKE CONCAT(");
        qb.push_bind(words[i].to_string());
        qb.push(",'%')) ");
    }
    qb.push(";");
    let result: Vec<Word> = qb.build_query_as().fetch_all(&mut *conn).await.expect("db error");
    let mut by_verse = Vec::new();
    for (key, chunk) in &result.into_iter().chunk_by(|w| (w.book_number, w.chapter_number, w.verse_number)) {
        by_verse.push((key, chunk.collect()));
    }

    let mut book_names: Vec<String> = Vec::new();
    for verse in &by_verse {
        let bk_name: (String,) = sqlx::query_as("SELECT short_name FROM books WHERE book_number=?")
            .bind(verse.0.0)
            .fetch_one(&mut *conn)
            .await
            .unwrap();
        book_names.push(bk_name.0);
    }
    let by_verse_with_book_name = zip(by_verse.into_iter(), book_names.into_iter())
        .map(|(verse, book)| {
            ((book, verse.0.0, verse.0.1, verse.0.2), verse.1)
        }).collect();
    Ok(by_verse_with_book_name)
}

// }}}
// {{{ Get list of books, their color codes and names.
#[derive(FromRow)]
#[allow(dead_code)]
pub struct Book {
    pub book_color: String,
    pub book_number: u16,
    pub short_name: String,
    pub long_name: String,
}

pub async fn get_books(
    conn: &mut SqliteConnection,
) -> Result<Vec<Book>, sqlx::Error> {
    Ok(query_as("SELECT * FROM books")
        .fetch_all(conn)
        .await?)
}

// }}}
// {{{ Query books chapter count by book short_name. 
pub async fn get_book_chapter_count(
    conn: &mut SqliteConnection,
    book: &str,
) -> Result<u16, sqlx::Error> {
    let chapter_count: (u16,) = query_as(
        "SELECT chapter FROM verses JOIN books ON books.book_number=verses.book_number WHERE short_name=? GROUP BY chapter ORDER BY chapter DESC LIMIT 1;")
        .bind(book)
        .fetch_one(conn)
        .await.expect("database error");
    Ok(chapter_count.0)
}

// }}}
// {{{ Query book long name by short name 
pub async fn get_book_long_name(
    conn: &mut SqliteConnection,
    short_name: &str,
) -> Result<String, sqlx::Error> {
    let long_name: Result<(String,), sqlx::Error> = query_as(
        "SELECT books.long_name FROM books WHERE short_name=? LIMIT 1")
        .bind(short_name)
        .fetch_one(conn)
        .await;
    Ok(long_name.unwrap().0)
}

// }}}
// {{{ Get verses of a chapter
pub async fn get_chapter(
    conn: &mut SqliteConnection,
    book_short_name: &str,
    chapter_nr: u16,
) -> Result<Vec<(u16, String)>, sqlx::Error> {
    // (verse number, verse content)
    let result: Vec<(u16, String)> = query_as("SELECT verse, text FROM verses v JOIN books b ON v.book_number=b.book_number WHERE short_name=? AND chapter=?;")
        .bind(book_short_name)
        .bind(chapter_nr)
        .fetch_all(conn)
        .await.unwrap();
    Ok(result)

}


// }}}

