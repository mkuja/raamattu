use std::time::Instant;
use serde_json::{from_str, from_value};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use axum::extract;
use axum::response::Html;
use axum::http::StatusCode;
use askama::Template;
use sqlx::SqliteConnection;
use std::sync::Arc;
use tantivy::collector::TopDocs;
use tantivy::{Searcher, TantivyDocument};
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy};


use tantivy::schema::{Schema, TextFieldIndexing, TextOptions, IndexRecordOption};
use tantivy::tokenizer::{LowerCaser, SimpleTokenizer};
use tantivy_tokenizer_api::TokenFilter;
use tantivy_stemmers;


#[derive(sqlx::FromRow, Debug, Clone)]
struct VerseFromDb {
    book_name: String,
    chapter: u64,
    verse: u64,
    text: String,
}

/// Query param
#[derive(Deserialize)]
pub struct SearchQuery {
    search: String,
}


#[derive(Template)]
#[template(path="search.jinja")]
pub struct SearchContext {
    results: Vec<Verse>,
}


/// A search result
#[derive(Deserialize, Serialize)]
pub struct Verse {
    kirja: String,
    luku: u64,
    jae: u64,
    text: String,
}


pub async fn build_index(
    conn: &mut SqliteConnection,
) -> Result<(Arc<Index>, Arc<IndexReader>), tantivy::error::TantivyError> {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("kirja", STRING | STORED); // Finnish for the sake of better
                                                             // query strings.
    schema_builder.add_u64_field("luku", INDEXED|STORED);
    schema_builder.add_u64_field("jae", STORED);
    schema_builder.add_text_field("teksti", TextOptions::default()
        .set_indexing_options(
            TextFieldIndexing::default()
            .set_tokenizer("suomi")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions)
            )
        .set_stored()
        );

    let schema = schema_builder.build();

    let book_name = schema.get_field("kirja").unwrap();
    let chapter = schema.get_field("luku").unwrap();
    let verse_ = schema.get_field("jae").unwrap();
    let text = schema.get_field("teksti").unwrap();

    // Create instance of stemmer tokenizer.
    let stemmer = tantivy_stemmers::StemmerTokenizer::new(
        tantivy_stemmers::algorithms::finnish,
    );
    let tokenizer = tantivy::tokenizer::TextAnalyzer::builder(
        stemmer.transform(LowerCaser.transform(SimpleTokenizer::default())),
    ).build();
    let index = Index::create_in_ram(schema.clone());
    index.tokenizers().register("suomi", tokenizer);

    let mut index_writer: IndexWriter = index.writer(50_000_000)?;

    let sql = "SELECT short_name as book_name, chapter, verse, text
        FROM verses JOIN books ON verses.book_number = books.book_number;";
    let verses: Vec<VerseFromDb> = sqlx::query_as(sql).fetch_all(&mut *conn).await.unwrap();

    for verse in verses {
        let doc = doc!(
            book_name => verse.book_name,
            chapter => verse.chapter,
            verse_ => verse.verse,
            text => verse.text,
        );
        index_writer.add_document(doc).unwrap();
    }
    index_writer.commit()?;

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    Ok((Arc::new(index), Arc::new(reader)))
}

pub async fn search_route(
    extract::State(state): extract::State<crate::ApplicationState>,
    search_query: extract::Query<SearchQuery>,
) -> Result<Html<String>, (StatusCode, String)> {
    let t1 = Instant::now();
    let text_field = state.index.schema().get_field("teksti").unwrap();

    let searcher = state.reader.searcher();
    let mut query_parser = QueryParser::for_index(&state.index, vec![text_field]);
    query_parser.set_conjunction_by_default();
    let query = query_parser.parse_query(&search_query.0.search);
    if query.is_err() {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, query.unwrap_err().to_string()));
    }
    let search_result = searcher.search(&query.unwrap(), &TopDocs::with_limit(300));
    if search_result.is_err() {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, search_result.unwrap_err().to_string()));
    }
    let search_result = search_result.unwrap();
    let mut search_context = SearchContext {
        results: Vec::new(),
    };
    for (_score, doc_addr) in search_result.into_iter() {
        let tantivy_doc: TantivyDocument = searcher.doc(doc_addr).unwrap();
        let verse_string = tantivy_doc.to_json(&state.index.schema());
        println!("{}", &verse_string);
        let mut verse: Value = from_str(&verse_string).unwrap();
        println!("KIRJA: {}", *verse.get("kirja").unwrap());
        let obj = verse.take();
        let mapped_vals = obj.as_object().unwrap();
        let book = mapped_vals["kirja"][0].clone();
        let chapter = mapped_vals["luku"][0].clone();
        let verse = mapped_vals["jae"][0].clone();
        let text = mapped_vals["teksti"][0].clone();
        
        search_context.results.push(
            Verse{
                kirja: from_value(book).unwrap(),
                luku: from_value(chapter).unwrap(),
                jae: from_value(verse).unwrap(),
                text: from_value(text).unwrap(),
            }
        );
    }

    let t2 = Instant::now();
    println!("Search took {:?}", t2-t1);

    Ok(Html(search_context.render().unwrap()))
}
