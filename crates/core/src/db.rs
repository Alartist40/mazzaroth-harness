use crate::content::ContentDocument;
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

#[derive(Clone)]
pub struct LibrarianDb {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub chunk_id: i64,
    pub doc_id: String,
    pub doc_title: String,
    pub category: String,
    pub license: String,
    pub retrieved_date: String,
    pub section_id: String,
    pub title_path: String,
    pub text: String,
    pub snippet: String,
    pub rank: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSummary {
    pub id: String,
    pub title: String,
    pub category: String,
    pub language: String,
    pub license: String,
    pub publisher: String,
    pub retrieved_date: String,
    pub personal_use_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: String,
    pub tier: String,
    pub label: String,
    pub content: String,
    pub tags: Vec<String>,
    pub strength: f32,
    pub activation: f32,
    pub access_count: i64,
    pub created_at: i64,
    pub last_accessed: i64,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLink {
    pub source_id: String,
    pub target_id: String,
    pub weight: f32,
    pub relationship: String,
    pub created_at: i64,
    pub last_reinforced: i64,
}

impl LibrarianDb {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let p = path.as_ref();
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(p)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        db.auto_migrate_vault(p)?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    pub fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // Enable WAL mode & foreign keys for high-concurrency embedded operation
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                category TEXT NOT NULL,
                language TEXT NOT NULL,
                license TEXT NOT NULL,
                source TEXT NOT NULL,
                publisher TEXT NOT NULL,
                license_url TEXT,
                retrieved_date TEXT NOT NULL,
                notes TEXT,
                personal_use_only INTEGER NOT NULL DEFAULT 0,
                raw_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS chunks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                doc_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
                section_id TEXT NOT NULL,
                title_path TEXT NOT NULL,
                text TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_chunks_doc_id ON chunks(doc_id);

            CREATE VIRTUAL TABLE IF NOT EXISTS chunks_fts USING fts5(
                text,
                title_path,
                doc_title,
                tokenize='porter unicode61'
            );

            CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                links TEXT NOT NULL DEFAULT '[]'
            );

            CREATE TABLE IF NOT EXISTS ingest_log (
                file_path TEXT PRIMARY KEY,
                file_hash TEXT NOT NULL,
                doc_id TEXT NOT NULL,
                ingested_at INTEGER NOT NULL,
                warnings TEXT
            );

            CREATE TABLE IF NOT EXISTS nodes (
                id TEXT PRIMARY KEY,
                tier TEXT NOT NULL DEFAULT 'celestial',
                label TEXT NOT NULL,
                content TEXT NOT NULL,
                tags TEXT NOT NULL DEFAULT '[]',
                strength REAL NOT NULL DEFAULT 1.0,
                activation REAL NOT NULL DEFAULT 1.0,
                access_count INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL DEFAULT 0,
                last_accessed INTEGER NOT NULL DEFAULT 0,
                pos_x REAL NOT NULL DEFAULT 0.0,
                pos_y REAL NOT NULL DEFAULT 0.0,
                pos_z REAL NOT NULL DEFAULT 0.0,
                vel_x REAL NOT NULL DEFAULT 0.0,
                vel_y REAL NOT NULL DEFAULT 0.0,
                vel_z REAL NOT NULL DEFAULT 0.0
            );

            CREATE TABLE IF NOT EXISTS links (
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                weight REAL NOT NULL DEFAULT 1.0,
                relationship TEXT NOT NULL DEFAULT 'associated',
                created_at INTEGER NOT NULL DEFAULT 0,
                last_reinforced INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (source_id, target_id)
            );

            -- Triggers to keep FTS5 in sync with chunks table
            CREATE TRIGGER IF NOT EXISTS chunks_ai AFTER INSERT ON chunks BEGIN
                INSERT INTO chunks_fts(rowid, text, title_path, doc_title)
                VALUES (new.id, new.text, new.title_path, (SELECT title FROM documents WHERE id = new.doc_id));
            END;

            CREATE TRIGGER IF NOT EXISTS chunks_ad AFTER DELETE ON chunks BEGIN
                DELETE FROM chunks_fts WHERE rowid = old.id;
            END;
            "#,
        )?;

        Ok(())
    }

    /// Auto-migrates orphaned legacy vault data from data/mazzaroth.db if needed
    fn auto_migrate_vault(&self, current_path: &Path) -> Result<()> {
        let node_count = self.count_nodes()?;
        if node_count == 0 {
            let legacy_path = Path::new("data/mazzaroth.db");
            if legacy_path.exists() && legacy_path != current_path {
                info!("Importing 1,097-node vault from data/mazzaroth.db");
                if let Ok(legacy_conn) = Connection::open(legacy_path) {
                    let mut stmt = legacy_conn.prepare("SELECT id, tier, label, content, tags, strength, activation, access_count, created_at, last_accessed, pos_x, pos_y, pos_z, vel_x, vel_y, vel_z FROM nodes")?;
                    let nodes_iter = stmt.query_map([], |row| {
                        Ok(MemoryNode {
                            id: row.get(0)?,
                            tier: row.get(1)?,
                            label: row.get(2)?,
                            content: row.get(3)?,
                            tags: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                            strength: row.get(5)?,
                            activation: row.get(6)?,
                            access_count: row.get(7)?,
                            created_at: row.get(8)?,
                            last_accessed: row.get(9)?,
                            pos_x: row.get(10)?,
                            pos_y: row.get(11)?,
                            pos_z: row.get(12)?,
                            vel_x: row.get(13)?,
                            vel_y: row.get(14)?,
                            vel_z: row.get(15)?,
                        })
                    })?;

                    for n in nodes_iter.filter_map(|r| r.ok()) {
                        let _ = self.insert_node(&n);
                    }

                    if let Ok(mut link_stmt) = legacy_conn.prepare("SELECT source_id, target_id, weight, relationship, created_at, last_reinforced FROM links") {
                        let links_iter = link_stmt.query_map([], |row| {
                            Ok(MemoryLink {
                                source_id: row.get(0)?,
                                target_id: row.get(1)?,
                                weight: row.get(2)?,
                                relationship: row.get(3)?,
                                created_at: row.get(4)?,
                                last_reinforced: row.get(5)?,
                            })
                        })?;
                        for l in links_iter.filter_map(|r| r.ok()) {
                            let _ = self.insert_link(&l);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Idempotent ingestion of document file
    pub fn ingest_file(&self, path: impl AsRef<Path>) -> Result<Option<String>> {
        let p = path.as_ref();
        let raw = std::fs::read_to_string(p)
            .with_context(|| format!("Failed to read file {}", p.display()))?;

        // 1. Compute SHA-256 hash for idempotency check
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        let conn = self.conn.lock().unwrap();

        // 2. Check if already ingested with identical hash
        let mut check_stmt = conn.prepare("SELECT file_hash FROM ingest_log WHERE file_path = ?1")?;
        let existing_hash: Option<String> = check_stmt
            .query_row(params![p.to_string_lossy()], |row| row.get(0))
            .ok();

        if let Some(h) = existing_hash {
            if h == hash {
                return Ok(None); // Skip idempotent re-ingestion
            }
        }

        // 3. Parse and validate provenance strictly
        let doc: ContentDocument = serde_json::from_str(&raw)
            .with_context(|| format!("Invalid JSON structure in {}", p.display()))?;

        doc.validate_provenance()
            .with_context(|| format!("Provenance validation failed for {}", p.display()))?;

        // 4. Ingest into documents, chunks, and log within a single transaction
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        conn.execute_batch("BEGIN TRANSACTION;")?;

        conn.execute(
            r#"
            INSERT OR REPLACE INTO documents (
                id, title, category, language, license, source, publisher, license_url, retrieved_date, notes, personal_use_only, raw_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
            params![
                doc.id,
                doc.title,
                doc.category,
                doc.language,
                doc.provenance.license,
                doc.provenance.source,
                doc.provenance.publisher,
                doc.provenance.license_url,
                doc.provenance.retrieved_date,
                doc.provenance.notes,
                if doc.provenance.personal_use_only { 1 } else { 0 },
                raw
            ],
        )?;

        // Remove old chunks if replacing
        conn.execute("DELETE FROM chunks WHERE doc_id = ?1", params![doc.id])?;

        let chunks = doc.to_chunks();
        {
            let mut chunk_stmt = conn.prepare_cached(
                "INSERT INTO chunks (doc_id, section_id, title_path, text) VALUES (?1, ?2, ?3, ?4)"
            )?;
            for chunk in &chunks {
                chunk_stmt.execute(params![chunk.doc_id, chunk.section_id, chunk.title_path, chunk.text])?;
            }
        }

        conn.execute(
            r#"
            INSERT OR REPLACE INTO ingest_log (file_path, file_hash, doc_id, ingested_at, warnings)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![p.to_string_lossy(), hash, doc.id, now, Option::<String>::None],
        )?;

        conn.execute_batch("COMMIT;")?;

        info!(doc_id = %doc.id, chunks = chunks.len(), "Ingested document successfully");
        Ok(Some(doc.id))
    }

    pub fn ingest_directory(&self, dir: impl AsRef<Path>) -> Result<usize> {
        let mut count = 0;
        let d = dir.as_ref();
        if !d.exists() {
            return Ok(0);
        }

        for entry in std::fs::read_dir(d)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "json") {
                match self.ingest_file(&path) {
                    Ok(Some(_)) => count += 1,
                    Ok(None) => {},
                    Err(e) => warn!(path = ?path, error = %e, "Skipped invalid document during ingestion"),
                }
            }
        }

        Ok(count)
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>> {
        let conn = self.conn.lock().unwrap();

        let sanitized: Vec<String> = query
            .split_whitespace()
            .filter(|w| {
                let lower = w.to_lowercase();
                !["what", "how", "why", "where", "when", "who", "is", "are", "the", "a", "an", "do", "does", "did", "can", "in", "to", "for", "of", "and", "or", "on"].contains(&lower.as_str())
            })
            .map(|w| {
                let clean: String = w.chars().filter(|c| c.is_alphanumeric()).collect();
                if clean.is_empty() { String::new() } else { format!("\"{}\"*", clean) }
            })
            .filter(|w| !w.is_empty())
            .collect();

        let fts_query = if sanitized.is_empty() {
            let fallback: Vec<String> = query
                .split_whitespace()
                .map(|w| format!("\"{}\"*", w.replace('"', "")))
                .collect();
            if fallback.is_empty() {
                return Ok(Vec::new());
            }
            fallback.join(" OR ")
        } else {
            sanitized.join(" OR ")
        };

        let mut stmt = conn.prepare(
            r#"
            SELECT 
                c.id, c.doc_id, d.title, d.category, d.license, d.retrieved_date,
                c.section_id, c.title_path, c.text,
                snippet(chunks_fts, 0, '<b>', '</b>', '...', 24) as snippet,
                bm25(chunks_fts) as rank
            FROM chunks_fts
            JOIN chunks c ON chunks_fts.rowid = c.id
            JOIN documents d ON c.doc_id = d.id
            WHERE chunks_fts MATCH ?1
            ORDER BY rank
            LIMIT ?2
            "#,
        )?;

        let rows = stmt.query_map(params![fts_query, limit], |row| {
            Ok(SearchHit {
                chunk_id: row.get(0)?,
                doc_id: row.get(1)?,
                doc_title: row.get(2)?,
                category: row.get(3)?,
                license: row.get(4)?,
                retrieved_date: row.get(5)?,
                section_id: row.get(6)?,
                title_path: row.get(7)?,
                text: row.get(8)?,
                snippet: row.get(9)?,
                rank: row.get(10)?,
            })
        })?;

        let mut hits = Vec::new();
        for r in rows {
            hits.push(r?);
        }

        Ok(hits)
    }

    pub fn list_documents(&self) -> Result<Vec<DocumentSummary>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, category, language, license, publisher, retrieved_date, personal_use_only FROM documents ORDER BY title",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(DocumentSummary {
                id: row.get(0)?,
                title: row.get(1)?,
                category: row.get(2)?,
                language: row.get(3)?,
                license: row.get(4)?,
                publisher: row.get(5)?,
                retrieved_date: row.get(6)?,
                personal_use_only: row.get::<_, i64>(7)? != 0,
            })
        })?;

        let mut docs = Vec::new();
        for r in rows {
            docs.push(r?);
        }
        Ok(docs)
    }

    pub fn get_document(&self, doc_id: &str) -> Result<Option<ContentDocument>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT raw_json FROM documents WHERE id = ?1")?;
        let raw: Option<String> = stmt.query_row(params![doc_id], |r| r.get(0)).ok();

        match raw {
            Some(json_str) => {
                let doc: ContentDocument = serde_json::from_str(&json_str)?;
                Ok(Some(doc))
            }
            None => Ok(None),
        }
    }

    pub fn list_categories(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT DISTINCT category FROM documents ORDER BY category")?;
        let rows = stmt.query_map([], |r| r.get(0))?;
        let mut cats = Vec::new();
        for r in rows {
            cats.push(r?);
        }
        Ok(cats)
    }

    pub fn count_documents(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT count(*) FROM documents", [], |r| r.get(0))?;
        Ok(count as usize)
    }

    pub fn count_chunks(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT count(*) FROM chunks", [], |r| r.get(0))?;
        Ok(count as usize)
    }

    pub fn insert_node(&self, node: &MemoryNode) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let tags_json = serde_json::to_string(&node.tags).unwrap_or_else(|_| "[]".to_string());
        conn.execute(
            r#"
            INSERT OR REPLACE INTO nodes (
                id, tier, label, content, tags, strength, activation, access_count,
                created_at, last_accessed, pos_x, pos_y, pos_z, vel_x, vel_y, vel_z
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
            "#,
            params![
                node.id, node.tier, node.label, node.content, tags_json,
                node.strength, node.activation, node.access_count,
                node.created_at, node.last_accessed,
                node.pos_x, node.pos_y, node.pos_z,
                node.vel_x, node.vel_y, node.vel_z
            ],
        )?;
        Ok(())
    }

    pub fn insert_link(&self, link: &MemoryLink) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            r#"
            INSERT OR REPLACE INTO links (source_id, target_id, weight, relationship, created_at, last_reinforced)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![link.source_id, link.target_id, link.weight, link.relationship, link.created_at, link.last_reinforced],
        )?;
        Ok(())
    }

    pub fn list_nodes(&self) -> Result<Vec<MemoryNode>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, tier, label, content, tags, strength, activation, access_count, created_at, last_accessed, pos_x, pos_y, pos_z, vel_x, vel_y, vel_z FROM nodes")?;
        let rows = stmt.query_map([], |row| {
            let tags_str: String = row.get(4)?;
            Ok(MemoryNode {
                id: row.get(0)?,
                tier: row.get(1)?,
                label: row.get(2)?,
                content: row.get(3)?,
                tags: serde_json::from_str(&tags_str).unwrap_or_default(),
                strength: row.get(5)?,
                activation: row.get(6)?,
                access_count: row.get(7)?,
                created_at: row.get(8)?,
                last_accessed: row.get(9)?,
                pos_x: row.get(10)?,
                pos_y: row.get(11)?,
                pos_z: row.get(12)?,
                vel_x: row.get(13)?,
                vel_y: row.get(14)?,
                vel_z: row.get(15)?,
            })
        })?;

        let mut nodes = Vec::new();
        for r in rows {
            nodes.push(r?);
        }
        Ok(nodes)
    }

    pub fn list_links(&self) -> Result<Vec<MemoryLink>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT source_id, target_id, weight, relationship, created_at, last_reinforced FROM links")?;
        let rows = stmt.query_map([], |row| {
            Ok(MemoryLink {
                source_id: row.get(0)?,
                target_id: row.get(1)?,
                weight: row.get(2)?,
                relationship: row.get(3)?,
                created_at: row.get(4)?,
                last_reinforced: row.get(5)?,
            })
        })?;

        let mut links = Vec::new();
        for r in rows {
            links.push(r?);
        }
        Ok(links)
    }

    pub fn count_nodes(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT count(*) FROM nodes", [], |r| r.get(0))?;
        Ok(count as usize)
    }

    pub fn rebuild_index(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("INSERT INTO chunks_fts(chunks_fts) VALUES('rebuild')", [])?;
        conn.execute("VACUUM", [])?;
        Ok(())
    }

    pub fn check_integrity(&self) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let result: String = conn.query_row("PRAGMA integrity_check", [], |r| r.get(0))?;
        Ok(result == "ok")
    }
}
