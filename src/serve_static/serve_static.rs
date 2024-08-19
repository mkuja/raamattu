use axum::routing::Router;
use tower_http::services::ServeDir;

pub fn serve_static() -> Router {
    Router::new().nest_service("/", ServeDir::new("static"))
}

