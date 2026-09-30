use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn is_safe_identifier(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptureBook {
    pub name: String,
    pub chapters: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeta {
    pub name: String,
    pub chapters: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptureMetaResponse {
    pub language: String,
    pub version: String,
    pub total_books: usize,
    pub books: Vec<BookMeta>,
}

#[derive(Clone)]
pub struct ScriptureReader {
    bibles_dir: PathBuf,
    cache: Arc<Mutex<HashMap<String, Arc<Vec<ScriptureBook>>>>>,
}

impl ScriptureReader {
    pub fn new(bibles_dir: impl AsRef<Path>) -> Self {
        Self {
            bibles_dir: bibles_dir.as_ref().to_path_buf(),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn resolve_file(&self, lang: &str, version: &str) -> Option<PathBuf> {
        if !is_safe_identifier(lang) || !is_safe_identifier(version) {
            return None;
        }

        let lang_dir = self.bibles_dir.join(lang);
        if !lang_dir.exists() {
            return None;
        }

        // Ensure canonical path stays strictly under bibles_dir
        let candidate = lang_dir.join(format!("{}.json", version));
        if candidate.exists() {
            return Some(candidate);
        }

        // Case-insensitive stem matching
        let v_lower = version.to_lowercase();
        if let Ok(entries) = std::fs::read_dir(&lang_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if stem.to_lowercase() == v_lower {
                            return Some(path);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn load_version(&self, lang: &str, version: &str) -> Result<Arc<Vec<ScriptureBook>>> {
        if !is_safe_identifier(lang) || !is_safe_identifier(version) {
            bail!("Invalid language or version identifier");
        }

        let key = format!("{}:{}", lang.to_lowercase(), version.to_lowercase());
        {
            let cache = self.cache.lock().unwrap();
            if let Some(books) = cache.get(&key) {
                return Ok(Arc::clone(books));
            }
        }

        let file_path = self
            .resolve_file(lang, version)
            .ok_or_else(|| anyhow::anyhow!("Scripture version not found: {}/{}", lang, version))?;

        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let books: Vec<ScriptureBook> = serde_json::from_reader(reader)?;
        let arc_books = Arc::new(books);

        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, Arc::clone(&arc_books));
        Ok(arc_books)
    }

    pub fn get_metadata(&self, lang: &str, version: &str) -> Result<ScriptureMetaResponse> {
        let books = self.load_version(lang, version)?;
        let meta_books: Vec<BookMeta> = books
            .iter()
            .map(|b| BookMeta {
                name: b.name.clone(),
                chapters: b.chapters.len(),
            })
            .collect();

        Ok(ScriptureMetaResponse {
            language: lang.to_string(),
            version: version.to_string(),
            total_books: meta_books.len(),
            books: meta_books,
        })
    }

    pub fn get_chapter(
        &self,
        lang: &str,
        version: &str,
        book_name: &str,
        chapter_num: usize,
    ) -> Result<Vec<String>> {
        let books = self.load_version(lang, version)?;
        let book = books
            .iter()
            .find(|b| b.name.eq_ignore_ascii_case(book_name))
            .ok_or_else(|| anyhow::anyhow!("Book not found: {}", book_name))?;

        let chapter_idx = if chapter_num > 0 { chapter_num - 1 } else { 0 };
        let verses = book
            .chapters
            .get(chapter_idx)
            .ok_or_else(|| anyhow::anyhow!("Chapter {} not found in book {}", chapter_num, book_name))?;

        Ok(verses.clone())
    }

    pub fn list_languages(&self) -> Vec<String> {
        let mut langs = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.bibles_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry.file_type().map_or(false, |t| t.is_dir()) {
                    if let Some(name) = entry.file_name().to_str() {
                        if is_safe_identifier(name) {
                            langs.push(name.to_string());
                        }
                    }
                }
            }
        }
        langs.sort();
        langs
    }

    pub fn list_versions(&self, lang: &str) -> Vec<String> {
        if !is_safe_identifier(lang) {
            return Vec::new();
        }

        let mut versions = Vec::new();
        let lang_dir = self.bibles_dir.join(lang);
        if let Ok(entries) = std::fs::read_dir(&lang_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if is_safe_identifier(stem) {
                            versions.push(stem.to_string());
                        }
                    }
                }
            }
        }
        versions.sort();
        versions
    }
}
