use librarian_core::LibrarianDb;
use tempfile::tempdir;

#[test]
fn test_provenance_validation_and_idempotent_ingest() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = LibrarianDb::open(&db_path).unwrap();

    // 1. Ingest valid document
    let valid_json = r#"{
        "id": "fm-21-76-survival",
        "title": "US Army Survival Manual FM 21-76",
        "category": "survival",
        "language": "en",
        "provenance": {
            "source": "US Government Printing Office",
            "publisher": "US Department of the Army",
            "license": "public-domain",
            "license_url": null,
            "retrieved_date": "2026-09-28",
            "notes": "1992 edition"
        },
        "structure": [
            {
                "id": "ch-01",
                "title": "Chapter 1: Introduction",
                "sections": [
                    {
                        "id": "ch-01-s-01",
                        "title": "Survival Actions",
                        "text": "Remember the keyword SURVIVAL. S - Size Up the Situation. U - Use All Your Senses. R - Remember Where You Are."
                    },
                    {
                        "id": "ch-01-s-02",
                        "title": "Pattern for Survival",
                        "text": "Develop a survival pattern that includes first aid, water, fire, food, and shelter."
                    }
                ]
            }
        ]
    }"#;

    let doc_file = dir.path().join("fm-21-76.json");
    std::fs::write(&doc_file, valid_json).unwrap();

    let res = db.ingest_file(&doc_file).unwrap();
    assert_eq!(res, Some("fm-21-76-survival".to_string()));

    // Verify document in DB
    assert_eq!(db.count_documents().unwrap(), 1);
    assert_eq!(db.count_chunks().unwrap(), 2);

    // 2. Idempotent check (same file -> Ok(None))
    let second_res = db.ingest_file(&doc_file).unwrap();
    assert_eq!(second_res, None, "Second ingest of identical file must return None (skipped)");
    assert_eq!(db.count_documents().unwrap(), 1);

    // 3. Search hits
    let hits = db.search("water", 5).unwrap();
    assert!(!hits.is_empty(), "Must find chunk mentioning water");
    assert_eq!(hits[0].doc_id, "fm-21-76-survival");
    assert!(hits[0].snippet.contains("water") || hits[0].snippet.contains("<b>"));

    // 4. Invalid provenance rejection (missing source)
    let invalid_json = r#"{
        "id": "invalid-doc",
        "title": "Bad Doc",
        "category": "test",
        "language": "en",
        "provenance": {
            "source": "",
            "publisher": "Someone",
            "license": "public-domain",
            "retrieved_date": "2026-09-28"
        },
        "structure": [
            { "id": "ch1", "title": "Ch 1", "sections": [] }
        ]
    }"#;
    let bad_file = dir.path().join("bad.json");
    std::fs::write(&bad_file, invalid_json).unwrap();
    let bad_res = db.ingest_file(&bad_file);
    assert!(bad_res.is_err(), "Must reject document with empty provenance source");

    // 5. Invalid license rejection
    let bad_license_json = r#"{
        "id": "bad-lic",
        "title": "Bad License",
        "category": "test",
        "language": "en",
        "provenance": {
            "source": "Web",
            "publisher": "Someone",
            "license": "all-rights-reserved",
            "retrieved_date": "2026-09-28"
        },
        "structure": [
            { "id": "ch1", "title": "Ch 1", "sections": [{ "id": "s1", "title": "S1", "text": "Hi" }] }
        ]
    }"#;
    let bad_lic_file = dir.path().join("bad_lic.json");
    std::fs::write(&bad_lic_file, bad_license_json).unwrap();
    let lic_res = db.ingest_file(&bad_lic_file);
    assert!(lic_res.is_err(), "Must reject document with unapproved license");
}
