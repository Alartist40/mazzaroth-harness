use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

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
    pub lang: String,
    pub version: String,
    pub books: Vec<BookMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptureChapterResponse {
    pub lang: String,
    pub version: String,
    pub book: String,
    pub chapter: usize,
    pub total_chapters: usize,
    pub verses: Vec<String>,
    pub text: String,
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
        let lang_dir = self.bibles_dir.join(lang);
        if !lang_dir.exists() {
            return None;
        }

        // Direct check
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

        {
            let mut cache = self.cache.lock().unwrap();
            // Bound cache size to 16 loaded versions
            if cache.len() > 16 {
                cache.clear();
            }
            cache.insert(key, Arc::clone(&arc_books));
        }

        Ok(arc_books)
    }

    pub fn get_meta(&self, lang: &str, version: &str) -> Result<ScriptureMetaResponse> {
        let books = self.load_version(lang, version)?;
        let meta_books: Vec<BookMeta> = books
            .iter()
            .map(|b| BookMeta {
                name: b.name.clone(),
                chapters: b.chapters.len(),
            })
            .collect();

        Ok(ScriptureMetaResponse {
            lang: lang.to_string(),
            version: version.to_string(),
            books: meta_books,
        })
    }

    pub fn get_chapter(
        &self,
        lang: &str,
        version: &str,
        book_name: &str,
        chapter_idx_1based: usize,
    ) -> Result<ScriptureChapterResponse> {
        let books = self.load_version(lang, version)?;
        let b_lower = book_name.to_lowercase();
        let book = books
            .iter()
            .find(|b| b.name.to_lowercase() == b_lower)
            .ok_or_else(|| anyhow::anyhow!("Book not found: {}", book_name))?;

        let total_chapters = book.chapters.len();
        if chapter_idx_1based == 0 || chapter_idx_1based > total_chapters {
            anyhow::bail!(
                "Chapter {} out of range (1..={}) for book {}",
                chapter_idx_1based,
                total_chapters,
                book_name
            );
        }

        let verses = &book.chapters[chapter_idx_1based - 1];
        let mut text_lines = Vec::with_capacity(verses.len());
        for (i, v) in verses.iter().enumerate() {
            text_lines.push(format!("{} {}", i + 1, v));
        }
        let text = text_lines.join("\n");

        Ok(ScriptureChapterResponse {
            lang: lang.to_string(),
            version: version.to_string(),
            book: book.name.clone(),
            chapter: chapter_idx_1based,
            total_chapters,
            verses: verses.clone(),
            text,
        })
    }

    pub fn list_languages(&self) -> Result<Vec<String>> {
        let mut langs = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.bibles_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry.path().is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        langs.push(name.to_string());
                    }
                }
            }
        }
        langs.sort();
        Ok(langs)
    }

    pub fn list_versions(&self, lang: &str) -> Result<Vec<String>> {
        let lang_dir = self.bibles_dir.join(lang);
        let mut versions = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&lang_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        versions.push(stem.to_string());
                    }
                }
            }
        }
        versions.sort();
        Ok(versions)
    }
}
