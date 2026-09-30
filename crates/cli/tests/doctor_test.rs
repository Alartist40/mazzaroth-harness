use librarian_core::{HardwareProfile, LibrarianDb};
use tempfile::tempdir;

#[test]
fn test_doctor_diagnostics() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("doctor_test.db");
    let db = LibrarianDb::open(&db_path).unwrap();

    let doc_json = r#"{
        "id": "doc1",
        "title": "Sample Doc",
        "category": "general",
        "language": "en",
        "provenance": {
            "source": "Gov",
            "publisher": "Gov",
            "license": "public-domain",
            "retrieved_date": "2026-09-28"
        },
        "structure": [
            { "id": "ch1", "title": "Ch 1", "sections": [{ "id": "s1", "title": "S 1", "text": "Content" }] }
        ]
    }"#;
    let doc_file = dir.path().join("d1.json");
    std::fs::write(&doc_file, doc_json).unwrap();
    db.ingest_file(&doc_file).unwrap();

    assert_eq!(db.count_documents().unwrap(), 1);
    assert_eq!(db.count_chunks().unwrap(), 1);

    let profile: HardwareProfile = "standard".parse().unwrap();
    assert_eq!(profile, HardwareProfile::Standard);
    assert!(profile.supports_llm());
    assert_eq!(profile.top_k(), 6);

    let tiny_profile: HardwareProfile = "tiny".parse().unwrap();
    assert_eq!(tiny_profile, HardwareProfile::Tiny);
    assert!(!tiny_profile.supports_llm());
}
