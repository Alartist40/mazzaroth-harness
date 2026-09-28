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
    /// Ingests Bible editions from the bibles directory into Mazzaroth's 3D Spiral Galaxy
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

        info!(total_languages = lang_dirs.len(), "Constructing logarithmic spiral celestial memory galaxy");

        let mut nodes_to_insert = Vec::new();
        let mut links_to_insert = Vec::new();

        // 1. Central Supermassive Galactic Core (The Universal Database Sun)
        let core_node = MemoryNode {
            id: "celestial:core:database".to_string(),
            tier: MemoryTier::Celestial,
            label: "🌟 Mazzaroth Universal Corpus Core".to_string(),
            content: "The Supermassive Knowledge Core. Unifying 60+ human languages and over 200 sacred biblical versions in a sovereign celestial memory matrix.".to_string(),
            tags: vec!["core".into(), "database".into(), "sovereignty".into()],
            strength: 1.0,
            activation: 1.0,
            access_count: 100,
            created_at: Self::now(),
            last_accessed: Self::now(),
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            vel_x: 0.0,
            vel_y: 0.0,
            vel_z: 0.0,
        };
        nodes_to_insert.push(core_node);
        total_nodes += 1;

        let num_spiral_arms = 4.0;
        let mut language_nodes = Vec::new();

        // 2. Language Superclusters (Planetary Stars along Spiral Arms)
        for (lang_idx, lang_path) in lang_dirs.iter().enumerate() {
            let lang_code = lang_path.file_name().unwrap_or_default().to_string_lossy().to_string();

            // Logarithmic Spiral matching visual galaxy dust: twist = 0.003, disc wave = sin(r * 0.01) * 20.0
            let arm_idx = (lang_idx as f32) % num_spiral_arms;
            let arm_offset = (arm_idx / num_spiral_arms) * std::f32::consts::PI * 2.0;
            // Geometric radial expansion: r_i = 90 * exp(0.033 * i) -> spreads from 90 to ~630 AU
            let dist_from_center = 90.0 * (0.033 * lang_idx as f32).exp();
            let spiral_angle = arm_offset + (dist_from_center * 0.003);

            let lang_x = spiral_angle.cos() * dist_from_center;
            let lang_z = spiral_angle.sin() * dist_from_center;
            let lang_y = (dist_from_center * 0.01).sin() * 20.0; // Galactic disc thickness wave aligned with dust

            let lang_sector_id = format!("celestial:lang:{}", lang_code);
            let sector_node = MemoryNode {
                id: lang_sector_id.clone(),
                tier: MemoryTier::Celestial,
                label: format!("🪐 Sector: {} ({})", lang_name_for_code(&lang_code), lang_code.to_uppercase()),
                content: format!("Galactic Linguistic Star for '{}'. Hub of all scriptural translations in this language.", lang_code),
                tags: vec!["language".into(), lang_code.clone()],
                strength: 0.95,
                activation: 0.9,
                access_count: 10,
                created_at: Self::now(),
                last_accessed: Self::now(),
                pos_x: lang_x,
                pos_y: lang_y,
                pos_z: lang_z,
                vel_x: 0.0,
                vel_y: 0.0,
                vel_z: 0.0,
            };

            nodes_to_insert.push(sector_node.clone());
            language_nodes.push((lang_code.clone(), lang_sector_id.clone()));
            total_nodes += 1;

            // Gravitational Ray to Core Sun
            links_to_insert.push(AssociativeLink {
                source_id: "celestial:core:database".to_string(),
                target_id: lang_sector_id.clone(),
                weight: 0.95,
                relationship: "core_gravitational_ray".to_string(),
                created_at: Self::now(),
                last_reinforced: Self::now(),
            });

            // 3. Bible Versions (Moons & Stars in close orbit around the Language Star)
            let json_files: Vec<PathBuf> = std::fs::read_dir(lang_path)?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
                .collect();

            for (v_idx, json_file) in json_files.iter().take(3).enumerate() {
                let version_name = json_file.file_stem().unwrap_or_default().to_string_lossy().to_string();
                let version_id = format!("semantic:version:{}:{}", lang_code, version_name);

                let v_orbit_angle = (v_idx as f32 * 2.1) + (lang_idx as f32 * 0.4);
                let v_orbit_radius = 28.0 + (v_idx as f32 * 12.0);

                let v_x = lang_x + v_orbit_angle.cos() * v_orbit_radius;
                let v_z = lang_z + v_orbit_angle.sin() * v_orbit_radius;
                let v_y = lang_y + ((v_idx as f32 * 5.0) - 5.0);

                let version_node = MemoryNode {
                    id: version_id.clone(),
                    tier: MemoryTier::Semantic,
                    label: format!("✦ Version: {}", version_name.to_uppercase()),
                    content: format!("Scriptural edition '{}' under language sector '{}'.", version_name, lang_code),
                    tags: vec!["version".into(), lang_code.clone(), version_name.clone()],
                    strength: 0.85,
                    activation: 0.85,
                    access_count: 5,
                    created_at: Self::now(),
                    last_accessed: Self::now(),
                    pos_x: v_x,
                    pos_y: v_y,
                    pos_z: v_z,
                    vel_x: 0.0,
                    vel_y: 0.0,
                    vel_z: 0.0,
                };

                nodes_to_insert.push(version_node);
                total_nodes += 1;

                // Orbit link to Language Planet
                links_to_insert.push(AssociativeLink {
                    source_id: lang_sector_id.clone(),
                    target_id: version_id.clone(),
                    weight: 0.85,
                    relationship: "version_orbit".to_string(),
                    created_at: Self::now(),
                    last_reinforced: Self::now(),
                });

                // 4. Books under this Version (Minor Asteroid / Planet Stars)
                if let Ok(file) = File::open(json_file) {
                    let reader = BufReader::new(file);
                    if let Ok(books) = serde_json::from_reader::<_, Vec<Book>>(reader) {
                        for (book_idx, book) in books.iter().take(max_books_per_version).enumerate() {
                            let book_node_id = format!("episodic:book:{}:{}:{}", lang_code, version_name, book.name.replace(' ', "_"));
                            
                            let book_angle = v_orbit_angle + (book_idx as f32 * 0.35);
                            let book_radius = 16.0 + (book_idx as f32 * 3.5);

                            let first_verse = book.chapters.first()
                                .and_then(|c| c.first())
                                .cloned()
                                .unwrap_or_default();

                            let sample_text: String = if first_verse.chars().count() > 140 {
                                format!("{}...", first_verse.chars().take(140).collect::<String>())
                            } else {
                                first_verse
                            };

                            let book_node = MemoryNode {
                                id: book_node_id.clone(),
                                tier: MemoryTier::Episodic,
                                label: format!("{} ({})", book.name, lang_code.to_uppercase()),
                                content: format!("{}: {}", book.name, sample_text),
                                tags: vec!["book".into(), lang_code.clone(), version_name.clone(), book.name.clone()],
                                strength: 0.7,
                                activation: 0.75,
                                access_count: 1,
                                created_at: Self::now(),
                                last_accessed: Self::now(),
                                pos_x: v_x + book_angle.cos() * book_radius,
                                pos_y: v_y + ((book_idx as f32 * 3.0) % 10.0) - 5.0,
                                pos_z: v_z + book_angle.sin() * book_radius,
                                vel_x: 0.0,
                                vel_y: 0.0,
                                vel_z: 0.0,
                            };

                            nodes_to_insert.push(book_node);
                            total_nodes += 1;

                            // Link book to Version
                            links_to_insert.push(AssociativeLink {
                                source_id: version_id.clone(),
                                target_id: book_node_id.clone(),
                                weight: 0.7,
                                relationship: "contains_chapter".to_string(),
                                created_at: Self::now(),
                                last_reinforced: Self::now(),
                            });
                        }
                    }
                }
            }
        }

        // 5. Cross-Linguistic Bridge Constellations (Interstellar Filaments along each spiral arm)
        for i in 0..language_nodes.len() {
            if i + 4 < language_nodes.len() {
                links_to_insert.push(AssociativeLink {
                    source_id: language_nodes[i].1.clone(),
                    target_id: language_nodes[i + 4].1.clone(),
                    weight: 0.6,
                    relationship: "interstellar_bridge".to_string(),
                    created_at: Self::now(),
                    last_reinforced: Self::now(),
                });
            }
        }

        info!(
            total_stars = nodes_to_insert.len(),
            total_constellation_filaments = links_to_insert.len(),
            "Writing spiral galaxy layout into SQLite store"
        );

        // Fast batch insert
        for node in &nodes_to_insert {
            let _ = engine.store.insert_node(node);
        }
        for link in &links_to_insert {
            let _ = engine.store.insert_link(link);
        }

        engine.sync_celestial_bodies()?;
        info!(total_stars = total_nodes, "Spiral Celestial Galaxy constructed successfully!");
        Ok(total_nodes)
    }

    fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }
}

fn lang_name_for_code(code: &str) -> &'static str {
    match code {
        "heb" => "Hebrew (עִבְרִית)",
        "ell" => "Greek (Ἑλληνική)",
        "lat" => "Latin (Latina)",
        "eng" => "English",
        "fra" => "French (Français)",
        "deu" | "ger" => "German (Deutsch)",
        "spa" => "Spanish (Español)",
        "rus" => "Russian (Русский)",
        "jpn" => "Japanese (日本語)",
        "zho" | "chi" => "Chinese (中文)",
        "hin" => "Hindi (हिन्दी)",
        "ara" => "Arabic (العربية)",
        "ita" => "Italian (Italiano)",
        "por" => "Portuguese (Português)",
        "kor" => "Korean (한국어)",
        "nor" => "Norwegian (Norsk)",
        "fin" => "Finnish (Suomi)",
        "swe" => "Swedish (Svenska)",
        "nld" => "Dutch (Nederlands)",
        "pol" => "Polish (Polski)",
        "tur" => "Turkish (Türkçe)",
        "tha" => "Thai (ไทย)",
        "vie" => "Vietnamese (Tiếng Việt)",
        "ceb" => "Cebuano",
        "tag" => "Tagalog",
        "swh" => "Swahili",
        "hun" => "Hungarian (Magyar)",
        "alb" => "Albanian (Shqip)",
        "ron" => "Romanian (Română)",
        "ces" => "Czech (Čeština)",
        "bul" => "Bulgarian (Български)",
        "ukr" => "Ukrainian (Українська)",
        "kat" => "Georgian (ქართული)",
        "hye" => "Armenian (Հայերեն)",
        "san" => "Sanskrit (संस्कृतम्)",
        "kan" => "Kannada (ಕನ್ನಡ)",
        "tam" => "Tamil (தமிழ்)",
        "tel" => "Telugu (తెలుగు)",
        "mal" => "Malayalam (മലയാളം)",
        "mar" => "Marathi (मराठी)",
        "guj" => "Gujarati (ગુજરાતી)",
        "pan" => "Punjabi (ਪੰਜਾਬੀ)",
        "ben" => "Bengali (বাংলা)",
        "urd" => "Urdu (اردو)",
        "ind" => "Indonesian (Bahasa)",
        "msa" => "Malay (Melayu)",
        "fil" => "Filipino",
        _ => "Ancient/Modern Tongue",
    }
}
