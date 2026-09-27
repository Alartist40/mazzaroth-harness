use crate::cognitive::node::{AssociativeLink, MemoryNode, MemoryTier};
use crate::engine::MazzarothEngine;
use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct Book {
    pub name: String,
    pub chapters: Vec<Vec<String>>,
}

pub struct CorpusImporter;

impl CorpusImporter {
    /// Ingests Bible editions from the bibles directory into Mazzaroth 3D Celestial Memory in a fast batch transaction
    pub fn import_bibles_directory(
        engine: &MazzarothEngine,
        bibles_dir: impl AsRef<Path>,
        max_languages: usize,
        max_books_per_version: usize,
    ) -> Result<usize> {
        let bibles_path = bibles_dir.as_ref();
        if !bibles_path.exists() {
            anyhow::bail!("Bibles directory does not exist: {:?}", bibles_path);
        }

        let mut total_nodes = 0;
        let mut lang_dirs: Vec<PathBuf> = std::fs::read_dir(bibles_path)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_dir())
            .collect();

        lang_dirs.sort();
        lang_dirs.truncate(max_languages);

        info!(total_languages = lang_dirs.len(), "Starting celestial corpus batch ingestion");

        let mut nodes_to_insert = Vec::new();
        let mut links_to_insert = Vec::new();

        for (lang_idx, lang_path) in lang_dirs.iter().enumerate() {
            let lang_code = lang_path.file_name().unwrap_or_default().to_string_lossy().to_string();

            // 1. Create Galactic Sector Supercluster Star for the Language
            let lang_sector_id = format!("celestial:sector:{}", lang_code);
            let angle = (lang_idx as f32 / max_languages.max(1) as f32) * std::f32::consts::PI * 2.0;
            let sector_dist = 220.0 + (lang_idx as f32 % 3.0) * 40.0;

            let sector_node = MemoryNode {
                id: lang_sector_id.clone(),
                tier: MemoryTier::Celestial,
                label: format!("Sector: {}", lang_code.to_uppercase()),
                content: format!("Galactic Linguistic Supercluster for language '{}'", lang_code),
                tags: vec!["corpus".into(), "language".into(), lang_code.clone()],
                strength: 1.0,
                activation: 0.95,
                access_count: 1,
                created_at: Self::now(),
                last_accessed: Self::now(),
                pos_x: angle.cos() * sector_dist,
                pos_y: ((lang_idx as f32 * 17.0) % 60.0) - 30.0,
                pos_z: angle.sin() * sector_dist,
                vel_x: 0.0,
                vel_y: 0.0,
                vel_z: 0.0,
            };

            nodes_to_insert.push(sector_node.clone());
            total_nodes += 1;

            links_to_insert.push(AssociativeLink {
                source_id: "celestial:core:identity".to_string(),
                target_id: lang_sector_id.clone(),
                weight: 0.9,
                relationship: "lingua_anchor".to_string(),
                created_at: Self::now(),
                last_reinforced: Self::now(),
            });

            // 2. Read first JSON version file for this language
            let json_files: Vec<PathBuf> = std::fs::read_dir(lang_path)?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
                .collect();

            if let Some(json_file) = json_files.first() {
                let version_name = json_file.file_stem().unwrap_or_default().to_string_lossy().to_string();
                
                if let Ok(file) = File::open(json_file) {
                    let reader = BufReader::new(file);
                    if let Ok(books) = serde_json::from_reader::<_, Vec<Book>>(reader) {
                        for (book_idx, book) in books.iter().take(max_books_per_version).enumerate() {
                            let book_node_id = format!("semantic:{}:{}:{}", lang_code, version_name, book.name.replace(' ', "_"));
                            
                            let book_angle = angle + (book_idx as f32 * 0.18);
                            let book_radius = 35.0 + (book_idx as f32 * 4.0);

                            let first_verse = book.chapters.first()
                                .and_then(|c| c.first())
                                .cloned()
                                .unwrap_or_default();

                            let sample_text: String = if first_verse.chars().count() > 150 {
                                format!("{}...", first_verse.chars().take(150).collect::<String>())
                            } else {
                                first_verse
                            };

                            let book_node = MemoryNode {
                                id: book_node_id.clone(),
                                tier: MemoryTier::Semantic,
                                label: format!("{} ({})", book.name, lang_code.to_uppercase()),
                                content: format!("{}: Chapter 1: {}", book.name, sample_text),
                                tags: vec!["bible".into(), lang_code.clone(), version_name.clone(), book.name.clone()],
                                strength: 0.85,
                                activation: 0.8,
                                access_count: 1,
                                created_at: Self::now(),
                                last_accessed: Self::now(),
                                pos_x: sector_node.pos_x + book_angle.cos() * book_radius,
                                pos_y: sector_node.pos_y + ((book_idx as f32 * 7.0) % 20.0) - 10.0,
                                pos_z: sector_node.pos_z + book_angle.sin() * book_radius,
                                vel_x: 0.0,
                                vel_y: 0.0,
                                vel_z: 0.0,
                            };

                            nodes_to_insert.push(book_node);
                            total_nodes += 1;

                            links_to_insert.push(AssociativeLink {
                                source_id: lang_sector_id.clone(),
                                target_id: book_node_id.clone(),
                                weight: 0.75,
                                relationship: "contains_book".to_string(),
                                created_at: Self::now(),
                                last_reinforced: Self::now(),
                            });
                        }
                    }
                }
            }
        }

        info!(total_prepared_nodes = nodes_to_insert.len(), "Writing batch into SQLite store");

        // Fast batch insert
        for node in &nodes_to_insert {
            let _ = engine.store.insert_node(node);
        }
        for link in &links_to_insert {
            let _ = engine.store.insert_link(link);
        }

        engine.sync_celestial_bodies()?;
        info!(total_stars = total_nodes, "Corpus batch ingestion finished. Galaxy ready!");
        Ok(total_nodes)
    }

    fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }
}
