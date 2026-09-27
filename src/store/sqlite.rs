use crate::cognitive::node::{AssociativeLink, MemoryNode, MemoryTier};
use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MazzarothStore {
    conn: Arc<Mutex<Connection>>,
}

impl MazzarothStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.init_schema()?;
        Ok(store)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;

            CREATE TABLE IF NOT EXISTS nodes (
                id TEXT PRIMARY KEY,
                tier TEXT NOT NULL,
                label TEXT NOT NULL,
                content TEXT NOT NULL,
                tags TEXT NOT NULL,
                strength REAL NOT NULL,
                activation REAL NOT NULL,
                access_count INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                last_accessed INTEGER NOT NULL,
                pos_x REAL NOT NULL,
                pos_y REAL NOT NULL,
                pos_z REAL NOT NULL,
                vel_x REAL NOT NULL,
                vel_y REAL NOT NULL,
                vel_z REAL NOT NULL
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS mazzaroth_fts USING fts5(
                id UNINDEXED,
                label,
                content,
                tags,
                tokenize = 'porter unicode61'
            );

            CREATE TABLE IF NOT EXISTS links (
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                weight REAL NOT NULL,
                relationship TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                last_reinforced INTEGER NOT NULL,
                PRIMARY KEY (source_id, target_id)
            );

            CREATE INDEX IF NOT EXISTS idx_nodes_tier ON nodes(tier);
            CREATE INDEX IF NOT EXISTS idx_nodes_activation ON nodes(activation);
            CREATE INDEX IF NOT EXISTS idx_links_source ON links(source_id);
            CREATE INDEX IF NOT EXISTS idx_links_target ON links(target_id);
            "#,
        )?;
        Ok(())
    }

    pub fn insert_node(&self, node: &MemoryNode) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let tags_json = serde_json::to_string(&node.tags).unwrap_or_else(|_| "[]".to_string());
        
        conn.execute(
            r#"
            INSERT OR REPLACE INTO nodes (
                id, tier, label, content, tags, strength, activation,
                access_count, created_at, last_accessed,
                pos_x, pos_y, pos_z, vel_x, vel_y, vel_z
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
            "#,
            params![
                node.id,
                node.tier.as_str(),
                node.label,
                node.content,
                tags_json,
                node.strength,
                node.activation,
                node.access_count as i64,
                node.created_at,
                node.last_accessed,
                node.pos_x,
                node.pos_y,
                node.pos_z,
                node.vel_x,
                node.vel_y,
                node.vel_z,
            ],
        )?;

        conn.execute(
            r#"
            INSERT OR REPLACE INTO mazzaroth_fts (id, label, content, tags)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            params![node.id, node.label, node.content, tags_json],
        )?;

        Ok(())
    }

    pub fn get_node(&self, id: &str) -> Result<Option<MemoryNode>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT id, tier, label, content, tags, strength, activation,
                   access_count, created_at, last_accessed,
                   pos_x, pos_y, pos_z, vel_x, vel_y, vel_z
            FROM nodes WHERE id = ?1
            "#,
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let tier_str: String = row.get(1)?;
            let tags_str: String = row.get(4)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            let tier = tier_str.parse::<MemoryTier>().ok().unwrap_or(MemoryTier::Episodic);

            Ok(Some(MemoryNode {
                id: row.get(0)?,
                tier,
                label: row.get(2)?,
                content: row.get(3)?,
                tags,
                strength: row.get(5)?,
                activation: row.get(6)?,
                access_count: row.get::<_, i64>(7)? as u64,
                created_at: row.get(8)?,
                last_accessed: row.get(9)?,
                pos_x: row.get(10)?,
                pos_y: row.get(11)?,
                pos_z: row.get(12)?,
                vel_x: row.get(13)?,
                vel_y: row.get(14)?,
                vel_z: row.get(15)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_nodes(&self) -> Result<Vec<MemoryNode>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT id, tier, label, content, tags, strength, activation,
                   access_count, created_at, last_accessed,
                   pos_x, pos_y, pos_z, vel_x, vel_y, vel_z
            FROM nodes
            ORDER BY activation DESC
            "#,
        )?;

        let rows = stmt.query_map([], |row| {
            let tier_str: String = row.get(1)?;
            let tags_str: String = row.get(4)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            let tier = tier_str.parse::<MemoryTier>().ok().unwrap_or(MemoryTier::Episodic);

            Ok(MemoryNode {
                id: row.get(0)?,
                tier,
                label: row.get(2)?,
                content: row.get(3)?,
                tags,
                strength: row.get(5)?,
                activation: row.get(6)?,
                access_count: row.get::<_, i64>(7)? as u64,
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

    pub fn search_fts(&self, query: &str, limit: usize) -> Result<Vec<MemoryNode>> {
        let conn = self.conn.lock().unwrap();
        let safe_query = query.replace(['"', '*'], "");
        let formatted = format!("\"{}\"*", safe_query);

        let mut stmt = conn.prepare(
            r#"
            SELECT n.id, n.tier, n.label, n.content, n.tags, n.strength, n.activation,
                   n.access_count, n.created_at, n.last_accessed,
                   n.pos_x, n.pos_y, n.pos_z, n.vel_x, n.vel_y, n.vel_z
            FROM mazzaroth_fts f
            JOIN nodes n ON f.id = n.id
            WHERE mazzaroth_fts MATCH ?1
            ORDER BY rank
            LIMIT ?2
            "#,
        )?;

        let rows = stmt.query_map(params![formatted, limit as i64], |row| {
            let tier_str: String = row.get(1)?;
            let tags_str: String = row.get(4)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            let tier = tier_str.parse::<MemoryTier>().ok().unwrap_or(MemoryTier::Episodic);

            Ok(MemoryNode {
                id: row.get(0)?,
                tier,
                label: row.get(2)?,
                content: row.get(3)?,
                tags,
                strength: row.get(5)?,
                activation: row.get(6)?,
                access_count: row.get::<_, i64>(7)? as u64,
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

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    pub fn insert_link(&self, link: &AssociativeLink) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            r#"
            INSERT OR REPLACE INTO links (source_id, target_id, weight, relationship, created_at, last_reinforced)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                link.source_id,
                link.target_id,
                link.weight,
                link.relationship,
                link.created_at,
                link.last_reinforced,
            ],
        )?;
        Ok(())
    }

    pub fn get_all_links(&self) -> Result<Vec<AssociativeLink>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT source_id, target_id, weight, relationship, created_at, last_reinforced
            FROM links
            ORDER BY weight DESC
            "#,
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(AssociativeLink {
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
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM nodes", [], |r| r.get(0))?;
        Ok(count as usize)
    }
}
