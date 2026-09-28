use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use mazzaroth::{create_router, MazzarothEngine, ServerState};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_mcp_tools_and_jsonrpc_conformance() {
    let engine = MazzarothEngine::in_memory().unwrap();
    let state = ServerState::new(engine);
    let app = create_router(state);

    // 1. List MCP Tools
    let list_req = Request::builder()
        .uri("/api/mcp")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list"
        }).to_string()))
        .unwrap();

    let resp = app.clone().oneshot(list_req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let val: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(val["jsonrpc"], "2.0");
    assert!(val["result"]["tools"].as_array().unwrap().len() >= 3);

    // 2. Call mazzaroth_remember
    let remember_req = Request::builder()
        .uri("/api/mcp")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": "mazzaroth_remember",
                "arguments": {
                    "label": "Deep Space Telemetry",
                    "content": "Voyager 1 interstellar coordinates locked.",
                    "tier": "semantic",
                    "tags": ["space", "telemetry"]
                }
            }
        }).to_string()))
        .unwrap();

    let remember_resp = app.clone().oneshot(remember_req).await.unwrap();
    assert_eq!(remember_resp.status(), StatusCode::OK);

    // 3. Call mazzaroth_recall
    let recall_req = Request::builder()
        .uri("/api/mcp")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "mazzaroth_recall",
                "arguments": {
                    "query": "interstellar",
                    "limit": 5
                }
            }
        }).to_string()))
        .unwrap();

    let recall_resp = app.oneshot(recall_req).await.unwrap();
    assert_eq!(recall_resp.status(), StatusCode::OK);

    let recall_bytes = recall_resp.into_body().collect().await.unwrap().to_bytes();
    let recall_val: serde_json::Value = serde_json::from_slice(&recall_bytes).unwrap();
    assert!(recall_val["result"]["content"][0]["text"].as_str().unwrap().contains("Deep Space Telemetry"));
}
