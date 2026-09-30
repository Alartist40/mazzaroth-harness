use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use librarian_core::{LibrarianConfig, LibrarianDb};
use librarian_server::{create_app, ServerState};
use tempfile::tempdir;
use tower::ServiceExt;

#[tokio::test]
async fn test_galaxy_graph_endpoint() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("galaxy_test.db");
    let db = LibrarianDb::open(&db_path).unwrap();

    let doc_json = r#"{
        "id": "medical-handbook",
        "title": "First Aid Field Handbook",
        "category": "medical",
        "language": "en",
        "provenance": {
            "source": "US Gov",
            "publisher": "Gov Printing",
            "license": "public-domain",
            "retrieved_date": "2026-09-28"
        },
        "structure": [
            {
                "id": "ch1",
                "title": "Bleeding Control",
                "sections": [
                    { "id": "s1", "title": "Direct Pressure", "text": "Apply firm direct pressure with a clean cloth." }
                ]
            }
        ]
    }"#;

    let doc_file = dir.path().join("med.json");
    std::fs::write(&doc_file, doc_json).unwrap();
    db.ingest_file(&doc_file).unwrap();

    db.create_note("Tourniquet notes", "See [[medical-handbook#s1]] for details.").unwrap();

    let config = LibrarianConfig::default();
    let state = ServerState::new(db, config, None);
    let app = create_app(state);

    let req = Request::builder().uri("/api/galaxy").body(Body::empty()).unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let val: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let nodes = val["nodes"].as_array().unwrap();
    let links = val["links"].as_array().unwrap();

    assert!(nodes.iter().any(|n| n["id"] == "core:librarian"));
    assert!(nodes.iter().any(|n| n["id"] == "category:medical"));
    assert!(nodes.iter().any(|n| n["id"] == "doc:medical-handbook"));
    assert!(nodes.iter().any(|n| n["kind"] == "note"));

    assert!(links.iter().any(|l| l["source_id"] == "core:librarian" && l["target_id"] == "category:medical"));
    assert!(links.iter().any(|l| l["relationship"] == "cites"));
}
