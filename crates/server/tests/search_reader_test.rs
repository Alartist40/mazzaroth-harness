use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use librarian_core::{LibrarianConfig, LibrarianDb};
use librarian_server::{create_app, ServerState};
use tempfile::tempdir;
use tower::ServiceExt;

#[tokio::test]
async fn test_search_and_reader_endpoints() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("server_test.db");
    let db = LibrarianDb::open(&db_path).unwrap();

    let doc_json = r#"{
        "id": "survival-guide",
        "title": "Wilderness Survival Guide",
        "category": "survival",
        "language": "en",
        "provenance": {
            "source": "Field Office",
            "publisher": "Gov",
            "license": "public-domain",
            "retrieved_date": "2026-09-28"
        },
        "structure": [
            {
                "id": "ch1",
                "title": "Water Purification",
                "sections": [
                    {
                        "id": "sec1",
                        "title": "Boiling Technique",
                        "text": "Boiling is the most certain method to kill all disease-causing organisms. Bring water to a rolling boil for at least 1 minute."
                    }
                ]
            }
        ]
    }"#;

    let doc_file = dir.path().join("guide.json");
    std::fs::write(&doc_file, doc_json).unwrap();
    db.ingest_file(&doc_file).unwrap();

    let config = LibrarianConfig::default();
    let state = ServerState::new(db, config, None);
    let app = create_app(state);

    // 1. Test /api/categories
    let cat_req = Request::builder().uri("/api/categories").body(Body::empty()).unwrap();
    let cat_res = app.clone().oneshot(cat_req).await.unwrap();
    assert_eq!(cat_res.status(), StatusCode::OK);
    let cat_bytes = cat_res.into_body().collect().await.unwrap().to_bytes();
    let cats: Vec<String> = serde_json::from_slice(&cat_bytes).unwrap();
    assert_eq!(cats, vec!["survival".to_string()]);

    // 2. Test /api/documents
    let doc_req = Request::builder().uri("/api/documents").body(Body::empty()).unwrap();
    let doc_res = app.clone().oneshot(doc_req).await.unwrap();
    assert_eq!(doc_res.status(), StatusCode::OK);

    // 3. Test /api/read/survival-guide
    let read_req = Request::builder().uri("/api/read/survival-guide").body(Body::empty()).unwrap();
    let read_res = app.clone().oneshot(read_req).await.unwrap();
    assert_eq!(read_res.status(), StatusCode::OK);
    let read_bytes = read_res.into_body().collect().await.unwrap().to_bytes();
    let val: serde_json::Value = serde_json::from_slice(&read_bytes).unwrap();
    assert_eq!(val["title"], "Wilderness Survival Guide");
    assert_eq!(val["provenance"]["license"], "public-domain");

    // 4. Test /api/search?q=boiling
    let search_req = Request::builder().uri("/api/search?q=boiling").body(Body::empty()).unwrap();
    let search_res = app.clone().oneshot(search_req).await.unwrap();
    assert_eq!(search_res.status(), StatusCode::OK);
    let search_bytes = search_res.into_body().collect().await.unwrap().to_bytes();
    let search_val: Vec<serde_json::Value> = serde_json::from_slice(&search_bytes).unwrap();
    assert_eq!(search_val.len(), 1);
    assert_eq!(search_val[0]["doc_id"], "survival-guide");
    assert!(search_val[0]["snippet"].as_str().unwrap().contains("Boiling") || search_val[0]["snippet"].as_str().unwrap().contains("<b>"));
}
