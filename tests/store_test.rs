use mazzaroth::{AssociativeLink, MazzarothStore, MemoryNode, MemoryTier};

#[test]
fn test_sqlite_fts5_and_links() {
    let store = MazzarothStore::open_in_memory().unwrap();

    let node1 = MemoryNode::new("n1", MemoryTier::Semantic, "Quantum Physics", "Superposition and entanglement principles");
    let node2 = MemoryNode::new("n2", MemoryTier::Episodic, "Conversation with Xander", "Discussed quantum computing and robotic servos");

    store.insert_node(&node1).unwrap();
    store.insert_node(&node2).unwrap();

    // Link insertion
    let link = AssociativeLink {
        source_id: "n1".to_string(),
        target_id: "n2".to_string(),
        weight: 0.85,
        relationship: "references".to_string(),
        created_at: 1000,
        last_reinforced: 1000,
    };
    store.insert_link(&link).unwrap();

    // FTS5 BM25 search
    let search_res = store.search_fts("quantum", 10).unwrap();
    assert_eq!(search_res.len(), 2, "Both nodes mention quantum and must be returned by FTS5");

    let links = store.get_all_links().unwrap();
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].relationship, "references");
}
