use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use librarian_core::{LibrarianConfig, LibrarianDb};
use librarian_server::{create_app, ServerState};
use tempfile::tempdir;
use tower::ServiceExt;

#[tokio::test]
async fn test_sky_deck_and_tree_endpoints() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("server_sky_test.db");
    let db = LibrarianDb::open(&db_path).unwrap();

    let doc_json = r#"{
        "id": "star-navigation-handbook",
        "title": "Star Navigation & Celestial Lore Handbook",
        "category": "astronomy",
        "language": "en",
        "provenance": {
            "source": "Mazzaroth Astrometry Archive",
            "publisher": "Public Domain Astronomy Collective",
            "license": "public-domain",
            "retrieved_date": "2026-09-30"
        },
        "structure": [
            {
                "id": "chapter-1-northern-pointers",
                "title": "Chapter 1: The Northern Sky and the True Pole",
                "sections": [
                    {
                        "id": "sec-polaris-pointer",
                        "title": "Finding Polaris via the Big Dipper Pointer Stars",
                        "text": "Merak and Dubhe point directly to Polaris."
                    }
                ]
            }
        ]
    }"#;

    let doc_file = dir.path().join("star-nav.json");
    std::fs::write(&doc_file, doc_json).unwrap();
    db.ingest_file(&doc_file).unwrap();

    let config = LibrarianConfig::default();
    let state = ServerState::new(db, config, None);
    let app = create_app(state);

    // 1. Test /api/sections includes 5 sections including "sky"
    let sec_req = Request::builder().uri("/api/sections").body(Body::empty()).unwrap();
    let sec_res = app.clone().oneshot(sec_req).await.unwrap();
    assert_eq!(sec_res.status(), StatusCode::OK);
    let sec_bytes = sec_res.into_body().collect().await.unwrap().to_bytes();
    let sections: Vec<serde_json::Value> = serde_json::from_slice(&sec_bytes).unwrap();
    assert_eq!(sections.len(), 5);
    assert!(sections.iter().any(|s| s["id"] == "sky"));

    // 2. Test /api/sky default
    let sky_req = Request::builder().uri("/api/sky?lat=35.6762&lon=139.6503&time=2026-09-30T21:00:00Z").body(Body::empty()).unwrap();
    let sky_res = app.clone().oneshot(sky_req).await.unwrap();
    assert_eq!(sky_res.status(), StatusCode::OK);
    let sky_bytes = sky_res.into_body().collect().await.unwrap().to_bytes();
    let sky_val: serde_json::Value = serde_json::from_slice(&sky_bytes).unwrap();
    assert_eq!(sky_val["lat"], 35.6762);
    assert_eq!(sky_val["lon"], 139.6503);
    assert!(sky_val["visible_stars"].as_array().unwrap().len() > 10);
    assert!(sky_val["cardinal_bearings"].as_array().unwrap().len() == 8);

    // 3. Test /api/tree returns categorized hierarchy
    let tree_req = Request::builder().uri("/api/tree").body(Body::empty()).unwrap();
    let tree_res = app.clone().oneshot(tree_req).await.unwrap();
    assert_eq!(tree_res.status(), StatusCode::OK);
    let tree_bytes = tree_res.into_body().collect().await.unwrap().to_bytes();
    let tree_val: Vec<serde_json::Value> = serde_json::from_slice(&tree_bytes).unwrap();
    assert!(!tree_val.is_empty());
    assert_eq!(tree_val[0]["category"], "astronomy");
    assert_eq!(tree_val[0]["languages"][0]["language"], "en");
    assert_eq!(tree_val[0]["languages"][0]["documents"][0]["id"], "star-navigation-handbook");
    assert_eq!(tree_val[0]["languages"][0]["documents"][0]["chapters"][0]["id"], "chapter-1-northern-pointers");
}
