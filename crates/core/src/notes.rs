use crate::db::LibrarianDb;
use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub links: Vec<String>,
}

pub fn extract_backlinks(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut rest = content;
    while let Some(start) = rest.find("[[") {
        let after_start = &rest[start + 2..];
        if let Some(end) = after_start.find("]]") {
            let target = after_start[..end].trim();
            if !target.is_empty() && !links.contains(&target.to_string()) {
                links.push(target.to_string());
            }
            rest = &after_start[end + 2..];
        } else {
            break;
        }
    }
    links
}

impl LibrarianDb {
    pub fn create_note(&self, title: &str, content: &str) -> Result<Note> {
        let id = Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        let links = extract_backlinks(content);
        let links_json = serde_json::to_string(&links)?;

        let conn = self.conn.lock().unwrap();
        conn.execute(
            r#"
            INSERT INTO notes (id, title, content, created_at, updated_at, links)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![id, title, content, now, now, links_json],
        )?;

        Ok(Note {
            id,
            title: title.to_string(),
            content: content.to_string(),
            created_at: now,
            updated_at: now,
            links,
        })
    }

    pub fn update_note(&self, id: &str, title: &str, content: &str) -> Result<Option<Note>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        let links = extract_backlinks(content);
        let links_json = serde_json::to_string(&links)?;

        let conn = self.conn.lock().unwrap();
        let rows_affected = conn.execute(
            r#"
            UPDATE notes 
            SET title = ?1, content = ?2, updated_at = ?3, links = ?4
            WHERE id = ?5
            "#,
            params![title, content, now, links_json, id],
        )?;

        if rows_affected == 0 {
            return Ok(None);
        }

        let created_at: i64 = conn.query_row(
            "SELECT created_at FROM notes WHERE id = ?1",
            params![id],
            |r| r.get(0),
        )?;

        Ok(Some(Note {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            created_at,
            updated_at: now,
            links,
        }))
    }

    pub fn get_note(&self, id: &str) -> Result<Option<Note>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, content, created_at, updated_at, links FROM notes WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let links_str: String = row.get(5)?;
            let links: Vec<String> = serde_json::from_str(&links_str).unwrap_or_default();
            Ok(Some(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                links,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_notes(&self) -> Result<Vec<Note>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, content, created_at, updated_at, links FROM notes ORDER BY updated_at DESC")?;
        let rows = stmt.query_map([], |row| {
            let links_str: String = row.get(5)?;
            let links: Vec<String> = serde_json::from_str(&links_str).unwrap_or_default();
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                links,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn delete_note(&self, id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        Ok(rows > 0)
    }
}
