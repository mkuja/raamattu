use std::sync::Arc;
use sqlx::SqlitePool;
use tantivy::{Index, IndexReader};


#[derive(Clone)]
pub struct ApplicationState {
    pub pool: SqlitePool,
    pub index: Arc<Index>,
    pub reader: Arc<IndexReader>,
}


