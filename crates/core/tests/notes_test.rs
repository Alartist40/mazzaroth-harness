use librarian_core::LibrarianDb;

#[test]
fn test_notes_and_backlinks() {
    let db = LibrarianDb::open_in_memory().unwrap();

    let note = db
        .create_note(
            "Water Filtration Field Notes",
            "According to [[fm-21-76-survival#ch-01-s-02]] we need sand, charcoal, and grass for layering.",
        )
        .unwrap();

    assert_eq!(note.title, "Water Filtration Field Notes");
    assert_eq!(note.links, vec!["fm-21-76-survival#ch-01-s-02".to_string()]);

    let fetched = db.get_note(&note.id).unwrap().expect("Note must exist");
    assert_eq!(fetched.links.len(), 1);

    let updated = db
        .update_note(
            &note.id,
            "Updated Water Notes",
            "Refer to [[fm-21-76-survival#ch-01-s-02]] and also [[gutenberg-123#ch-02]].",
        )
        .unwrap()
        .expect("Must update");

    assert_eq!(updated.title, "Updated Water Notes");
    assert_eq!(updated.links.len(), 2);
    assert!(updated.links.contains(&"gutenberg-123#ch-02".to_string()));

    let list = db.list_notes().unwrap();
    assert_eq!(list.len(), 1);

    let deleted = db.delete_note(&note.id).unwrap();
    assert!(deleted);
    assert_eq!(db.list_notes().unwrap().len(), 0);
}
