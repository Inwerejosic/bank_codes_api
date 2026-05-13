use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    data: Arc<Value>,
}

#[derive(Serialize, Deserialize)]
struct QueryParams {
    #[serde(default)]
    path: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let json_data = match load_json_file("data.json").await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading JSON file: {}", e);
            json!({ "error": "Failed to load data.json" })
        }
    };

    let state = AppState {
        data: Arc::new(json_data),
    };

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/data", get(get_all_data))
        .route("/api/query", post(query_data))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("Server error");
}

async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

async fn get_all_data(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.data.as_ref().clone())
}

async fn query_data(
    State(state): State<AppState>,
    Json(params): Json<QueryParams>,
) -> Result<impl IntoResponse, QueryError> {
    if let Some(path) = params.path {
        let result = query_json(&state.data, &path)?;
        Ok(Json(json!({ "result": result })))
    } else {
        Ok(Json(state.data.as_ref().clone()))
    }
}

/// Query nested JSON using dot notation (e.g., "banks.0.name")
fn query_json(data: &Value, path: &str) -> Result<Value, QueryError> {
    let mut current = data;
    for key in path.split('.') {
        if let Ok(idx) = key.parse::<usize>() {
            // Array index access
            current = current
                .as_array()
                .and_then(|arr| arr.get(idx))
                .ok_or(QueryError::NotFound)?;
        } else {
            // Object key access
            current = current
                .as_object()
                .and_then(|obj| obj.get(key))
                .ok_or(QueryError::NotFound)?;
        }
    }
    Ok(current.clone())
}

async fn load_json_file(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = tokio::fs::read_to_string(path).await?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

#[derive(Debug)]
enum QueryError {
    NotFound,
}

impl IntoResponse for QueryError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::NOT_FOUND, Json(json!({ "error": "Path not found" }))).into_response()
    }
}

// Helper to extract State cleanly
use axum::extract::State;


// Inwerejosic - 2026