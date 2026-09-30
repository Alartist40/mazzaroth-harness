use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use librarian_core::{LibrarianConfig, LibrarianDb};
use librarian_server::{create_app, ServerState};
use tempfile::tempdir;
use tower::ServiceExt;

#[tokio::test]
async fn test_grounded_librarian_prompt_and_refusal() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("ask_test.db");
    let db = LibrarianDb::open(&db_path).unwrap();

    let doc_json = r#"{
        "id": "survival-water",
        "title": "Water Survival Manual",
        "category": "survival",
        "language": "en",
        "provenance": {
            "source": "US Gov Printing Office",
            "publisher": "Gov",
            "license": "public-domain",
            "retrieved_date": "2026-09-28"
        },
        "structure": [
            {
                "id": "ch1",
                "title": "Purification",
                "sections": [
                    { "id": "s1", "title": "Boiling", "text": "Bring water to a rolling boil for 1 full minute." }
                ]
            }
        ]
    }"#;

    let doc_file = dir.path().join("water.json");
    std::fs::write(&doc_file, doc_json).unwrap();
    db.ingest_file(&doc_file).unwrap();

    let config = LibrarianConfig::default();
    let state = ServerState::new(db, config, None);
    let app = create_app(state);

    // 1. Ask query with hits -> receives SSE citations stream
    let req = Request::builder()
        .uri("/api/ask")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"question": "How long should I boil water?"}"#))
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let sse_text = String::from_utf8_lossy(&bytes);
    assert!(sse_text.contains("citations"));
    assert!(sse_text.contains("survival-water"));

    // 2. Ask query with zero hits -> receives refusal "I don't have that in the library"
    let zero_req = Request::builder()
        .uri("/api/ask")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"question": "How to replace an automotive transmission fluid gasket?"}"#))
        .unwrap();

    let zero_res = app.oneshot(zero_req).await.unwrap();
    assert_eq!(zero_res.status(), StatusCode::OK);
    let zero_bytes = zero_res.into_body().collect().await.unwrap().to_bytes();
    let zero_text = String::from_utf8_lossy(&zero_bytes);
    assert!(zero_text.contains("I don't have that in the library"));
}
