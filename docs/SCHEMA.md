# SCHEMA.md — Content & Database Schemas

## 1. JSON Content Schema (Per Document)

```json
{
  "id": "fm-21-76-survival",
  "title": "US Army Survival Manual FM 21-76",
  "category": "survival",
  "language": "en",
  "provenance": {
    "source": "US Government Printing Office",
    "publisher": "US Department of the Army",
    "license": "public-domain",
    "license_url": "https://www.usa.gov/government-works",
    "retrieved_date": "2026-09-28",
    "notes": "1992 edition"
  },
  "structure": [
    {
      "id": "ch-01",
      "title": "Chapter 1: Introduction",
      "sections": [
        {
          "id": "ch-01-s-01",
          "title": "Survival Actions",
          "text": "..."
        }
      ]
    }
  ]
}
```

## 2. SQLite Database Schema

```sql
CREATE TABLE documents (
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

CREATE TABLE chunks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    doc_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    section_id TEXT NOT NULL,
    title_path TEXT NOT NULL,
    text TEXT NOT NULL
);

CREATE VIRTUAL TABLE chunks_fts USING fts5(
    text,
    title_path,
    doc_title,
    content='chunks',
    content_rowid='id',
    tokenize='porter unicode61'
);

CREATE TABLE notes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    links TEXT NOT NULL DEFAULT '[]'
);

CREATE TABLE ingest_log (
    file_path TEXT PRIMARY KEY,
    file_hash TEXT NOT NULL,
    doc_id TEXT NOT NULL,
    ingested_at INTEGER NOT NULL,
    warnings TEXT
);
```
