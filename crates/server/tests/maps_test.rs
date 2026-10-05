use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use librarian_core::{LibrarianConfig, LibrarianDb};
use librarian_server::{create_app, ServerState};
use tempfile::tempdir;
use tower::ServiceExt;

#[tokio::test]
async fn test_pmtiles_range_serving() {
    let dir = tempdir().unwrap();
    let maps_dir = dir.path().join("maps");
    std::fs::create_dir_all(&maps_dir).unwrap();

    let test_map_file = maps_dir.join("sample.pmtiles");
    let fake_data = b"PMTiles\x03fake_tile_data_for_range_request_testing_0123456789";
    std::fs::write(&test_map_file, fake_data).unwrap();

    let db = LibrarianDb::open_in_memory().unwrap();
    let mut config = LibrarianConfig::default();
    config.maps_dir = maps_dir;

    let state = ServerState::new(db, config, None);
    let app = create_app(state);

    // 1. List maps
    let list_req = Request::builder().uri("/api/maps").body(Body::empty()).unwrap();
    let list_res = app.clone().oneshot(list_req).await.unwrap();
    assert_eq!(list_res.status(), StatusCode::OK);
    let list_bytes = list_res.into_body().collect().await.unwrap().to_bytes();
    let list_val: Vec<serde_json::Value> = serde_json::from_slice(&list_bytes).unwrap();
    assert_eq!(list_val.len(), 1);
    assert_eq!(list_val[0]["filename"], "sample.pmtiles");

    // 2. HTTP Range request (bytes=0-6 -> "PMTiles")
    let range_req = Request::builder()
        .uri("/maps/sample.pmtiles")
        .header("Range", "bytes=0-6")
        .body(Body::empty())
        .unwrap();

    let range_res = app.clone().oneshot(range_req).await.unwrap();
    assert_eq!(range_res.status(), StatusCode::PARTIAL_CONTENT);
    let range_bytes = range_res.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&range_bytes[..], b"PMTiles");

    // 3. Suffix Range request (bytes=-10 -> last 10 bytes: "0123456789")
    let suffix_req = Request::builder()
        .uri("/maps/sample.pmtiles")
        .header("Range", "bytes=-10")
        .body(Body::empty())
        .unwrap();
    let suffix_res = app.clone().oneshot(suffix_req).await.unwrap();
    assert_eq!(suffix_res.status(), StatusCode::PARTIAL_CONTENT);
    let suffix_bytes = suffix_res.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&suffix_bytes[..], b"0123456789");

    // 4. Open Range request (bytes=0-)
    let open_req = Request::builder()
        .uri("/maps/sample.pmtiles")
        .header("Range", "bytes=0-")
        .body(Body::empty())
        .unwrap();
    let open_res = app.clone().oneshot(open_req).await.unwrap();
    assert_eq!(open_res.status(), StatusCode::PARTIAL_CONTENT);
    let open_bytes = open_res.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&open_bytes[..], fake_data);

    // 5. Zero-length file Range request
    let zero_file = dir.path().join("maps/empty.pmtiles");
    std::fs::write(&zero_file, b"").unwrap();
    let zero_req = Request::builder()
        .uri("/maps/empty.pmtiles")
        .header("Range", "bytes=0-10")
        .body(Body::empty())
        .unwrap();
    let zero_res = app.oneshot(zero_req).await.unwrap();
    assert_eq!(zero_res.status(), StatusCode::RANGE_NOT_SATISFIABLE);
}
