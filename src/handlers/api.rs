use axum::{extract::Path, Json};
use serde_json::{json, Value};

pub async fn current_metrics() -> Json<Value> {
    Json(json!({
        "data": {},
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "meta": { "type": "current" }
    }))
}

pub async fn history_metrics() -> Json<Value> {
    Json(json!({
        "data": [],
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "meta": { "type": "history" }
    }))
}

pub async fn specific_metric(Path(metric_type): Path<String>) -> Json<Value> {
    Json(json!({
        "data": {},
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "meta": { "type": "specific", "metric": metric_type }
    }))
}

pub async fn api_docs() -> Json<Value> {
    Json(json!({
        "endpoints": {
            "current": "GET /api/v1/metrics/current",
            "history": "GET /api/v1/metrics/history?range=1h|6h|24h|7d|30d",
            "specific": "GET /api/v1/metrics/{type}?range=X",
            "docs": "GET /api/v1/docs"
        }
    }))
}
