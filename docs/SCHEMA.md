# SCHEMA.md — Content, Database & API Schemas

This document defines all internal schemas, data models, database tables, and API contracts used throughout Mazzaroth (Offline Librarian Box).

---

## 1. JSON Content Schema (Ingestion Corpus)

Every document ingested into the knowledge vault via `mazzaroth ingest <dir>` must adhere to this structured JSON format with mandatory provenance.

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
    "notes": "1992 official field edition"
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

### Valid Provenance License Types
- `public-domain`
- `cc0`
- `cc-by`
- `cc-by-sa`
- `gutenberg`
- `personal-use-only`

---

## 2. SQLite Database Schema (`data/mazzaroth.db`)

The persistent storage is a single, zero-dependency SQLite 3 database operating with WAL mode, normal synchronous disk writes, and foreign key enforcement.

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA foreign_keys = ON;

-- Ingested Structured Documents
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

-- Searchable Document Chunks
CREATE TABLE IF NOT EXISTS chunks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    doc_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    section_id TEXT NOT NULL,
    title_path TEXT NOT NULL,
    text TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_chunks_doc_id ON chunks(doc_id);

-- Full-Text Search Virtual Table (FTS5 BM25)
CREATE VIRTUAL TABLE IF NOT EXISTS chunks_fts USING fts5(
    text,
    title_path,
    doc_title,
    tokenize='porter unicode61'
);

-- Field Notes & Backlink Knowledge Graph
CREATE TABLE IF NOT EXISTS notes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    links TEXT NOT NULL DEFAULT '[]'
);

-- SHA-256 Ingestion Idempotency Log
CREATE TABLE IF NOT EXISTS ingest_log (
    file_path TEXT PRIMARY KEY,
    file_hash TEXT NOT NULL,
    doc_id TEXT NOT NULL,
    ingested_at INTEGER NOT NULL,
    warnings TEXT
);

-- 3D Galaxy Graph Nodes
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

-- 3D Galaxy Graph Edges
CREATE TABLE IF NOT EXISTS links (
    source_id TEXT NOT NULL,
    target_id TEXT NOT NULL,
    weight REAL NOT NULL DEFAULT 1.0,
    relationship TEXT NOT NULL DEFAULT 'associated',
    created_at INTEGER NOT NULL DEFAULT 0,
    last_reinforced INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (source_id, target_id)
);
```

---

## 3. Astrometry & Celestial Data Models

### Star Record Schema
```rust
pub struct Star {
    pub id: &'static str,
    pub name: &'static str,
    pub constellation: &'static str,
    pub ra_hours: f64,       // Right Ascension [0.0..24.0]
    pub dec_deg: f64,        // Declination [-90.0..+90.0]
    pub mag: f64,            // Apparent visual magnitude
    pub spectral: &'static str,
}
```

### Astrometric Projection API Payload (`GET /api/sky`)
```json
{
  "lst_hours": 14.825,
  "lat": 35.6762,
  "lon": 139.6503,
  "time": "2026-09-30T13:00:00Z",
  "visible_stars": [
    {
      "id": "vega",
      "name": "Vega",
      "constellation": "Lyr",
      "alt": 64.28,
      "az": 285.12,
      "mag": 0.03,
      "spectral": "A0V",
      "x": -0.274,
      "y": 0.073
    }
  ],
  "lines": [
    {
      "constellation": "Lyr",
      "star_a": "vega",
      "star_b": "sheliak",
      "alt_a": 64.28,
      "az_a": 285.12,
      "alt_b": 58.14,
      "az_b": 281.45,
      "visible": true
    }
  ],
  "background_stars": [
    {
      "alt": 42.15,
      "az": 120.4,
      "mag": 4.5,
      "x": 0.458,
      "y": -0.264
    }
  ]
}
```

---

## 4. Knowledge Tree Hierarchy Schema (`GET /api/tree`)

```json
[
  {
    "category": "astronomy",
    "languages": [
      {
        "language": "en",
        "documents": [
          {
            "id": "star-navigation-handbook",
            "title": "Star Navigation & Celestial Lore Handbook",
            "publisher": "Navigational Astrometry Bureau",
            "license": "public-domain",
            "chapters": [
              {
                "id": "ch-01",
                "title": "Chapter 1: Fundamentals of Celestial Navigation",
                "sections": [
                  {
                    "id": "ch-01-s-01",
                    "title": "The Celestial Sphere and Terrestrial Coordinates"
                  }
                ]
              }
            ]
          }
        ]
      }
    ]
  }
]
```

---

## 5. Map & Downloader API Contracts

### Download Job Request (`POST /api/maps/fetch`)
```json
{
  "bbox": [139.5, 35.5, 140.0, 36.0],
  "name": "tokyo-core",
  "maxzoom": 14
}
```

### Download Job Status (`GET /api/maps/fetch`)
```json
{
  "state": "downloading",
  "progress": 45.2,
  "bytes_downloaded": 12451840,
  "total_bytes": 27520100,
  "filename": "tokyo-core.pmtiles",
  "log": [
    "Starting fetch for bbox [139.5, 35.5, 140.0, 36.0]",
    "Range extracting tiles up to zoom 14..."
  ]
}
```

