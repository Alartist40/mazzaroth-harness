use mazzaroth::{CorpusImporter, MazzarothEngine};
use std::path::Path;

#[test]
fn test_corpus_importer_and_galaxy_construction() {
    let engine = MazzarothEngine::in_memory().unwrap();
    let bibles_dir = Path::new("bibles");

    if bibles_dir.exists() {
        let count = CorpusImporter::import_bibles_directory(&engine, bibles_dir, 5, 3).unwrap();
        assert!(count > 0, "Importer must ingest language sectors and book stars");

        let galaxy = engine.get_galaxy_state().unwrap();
        assert!(!galaxy.bodies.is_empty(), "Galaxy bodies must be populated");
        assert!(!galaxy.lines.is_empty(), "Constellation links must connect sectors");

        // Verify FTS search across multilingual corpus
        let recalled = engine.recall("Genesis", 5).unwrap();
        assert!(!recalled.is_empty(), "Must recall Genesis nodes from corpus");
    }
}
