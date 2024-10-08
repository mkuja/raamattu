use chrono::{DateTime, Datelike, FixedOffset, NaiveDate};
use sitemap_rs::url::{ChangeFrequency, Url};
use sitemap_rs::url_set::UrlSet;
use sqlx::SqliteConnection;
use crate::db::query::{get_book_chapter_count, get_books};

pub async fn generate_sitemap(mut conn: &mut SqliteConnection) {
    let urls = urls(&mut conn).await;
    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    std::fs::write("./static/sitemap.xml", buf).expect("Could not write sitemap.xml")
}

async fn urls(mut conn: &mut SqliteConnection) -> Vec<Url> {
    let books = get_books(&mut conn).await;
    let short_book_names: Vec<String> = books
        .expect("Failed to generate sitemap")
        .into_iter()
        .map(|book| { book.short_name.clone() })
        .collect();

    let now = chrono::offset::Local::now().naive_utc();
    let mut urls = vec![
        Url::builder(String::from("https://raamattu.app/"))
            .last_modified(DateTime::from_naive_utc_and_offset(
                now,
                FixedOffset::east_opt(0).unwrap(),
            ))
            .change_frequency(ChangeFrequency::Monthly)
            .priority(1.0)
            .build()
            .expect("failed a <url> validation"),
    ];
    urls.extend(
        short_book_names.iter().map(|book| {
            return
                Url::builder(format!("https://raamattu.app/books/{book}"))
                    .last_modified(DateTime::from_naive_utc_and_offset(
                        now,
                        FixedOffset::east_opt(0).unwrap(),
                    ))
                    .change_frequency(ChangeFrequency::Monthly)
                    .priority(0.8)
                    .build()
                    .expect("failed a <url> validation");
        })
    );

    for book in short_book_names {
        let num_chapters = get_book_chapter_count(&mut conn, &book).await;
        for num_chapter in 1..=num_chapters.unwrap() {
            urls.push(Url::builder(format!("https://raamattu.app/books/{book}/{num_chapter}"))
                .last_modified(DateTime::from_naive_utc_and_offset(
                    now,
                    FixedOffset::east_opt(0).unwrap(),
                ))
                .change_frequency(ChangeFrequency::Monthly)
                .priority(0.6)
                .build()
                .expect("failed a <url> validation"))
        }
    }
    urls
}
