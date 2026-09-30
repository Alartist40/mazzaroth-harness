# Librarian Box (Mazzaroth) — Offline Sovereign Knowledge Vault

An offline librarian in a box. A single, self-contained Rust binary running on any ARM64 SBC (Raspberry Pi 4/5, Orange Pi 5) or desktop machine, serving a curated, legally clean knowledge library through a web interface, and answering questions with a local language model acting as a grounded librarian.

Zero Docker. Zero cloud. Works completely offline.

---

## Key Capabilities

1. **Curated & Provenance-Enforced Library**: Mandatory provenance metadata enforced at ingestion (source, publisher, license, date). Ingestion rejects any unverified or copyrighted text.
2. **Instant BM25 Search**: SQLite FTS5 search index with highlighted snippet context.
3. **Intuitive Category & Hierarchy Explorer**: Visual drilldown through knowledge domains: `Theme > Language > Document/Book > Chapter > Passage` with full interactive breadcrumbs and `#NEXUS-0` central indexing.
4. **Grounded AI Librarian**: Local LLM (Ministral 3B via Ollama / OpenAI-compatible endpoint) answers strictly from verified passages with exact citations and dates. Refuses out-of-context queries and degrades gracefully if the LLM is offline.
5. **3D & 2D Galaxy Graph**: Interactive 3D Three.js logarithmic particle spiral and lightweight 2D Canvas fallback visualizing the knowledge vault by category, document, and field note backlinks.
6. **Constellations & Celestial Dome**: Real-time celestial sphere projection and live alt/az astrometry dome (Julian Day, LST, 115+ navigational stars, 40+ constellation vector lines, 800+ background starfield, time/date & GPS HUD), plus 3D celestial sphere and 20+ poster grid catalog.
7. **Offline PMTiles Maps**: Direct HTTP Range byte-serving for single-file `.pmtiles` regional OpenStreetMap packages with street-level downloader.
8. **Field Notes & Star Navigation Handbook**: Markdown note-taking and public-domain star lore handbook with backlinks (`[[doc-id#section-id]]`) into the Deep Reader.

---

## System Architecture & 4 Workspaces

Mazzaroth operates as a unified 5-zone sovereign desktop web interface partitioned into 4 primary modules:

### 1. `01 GALAXY` (Database Vault Particle Spiral)
- **3D Interactive Orbit & Particle Core**: Renders 1,100+ database memory nodes and notes as luminous particle sprites along 5 logarithmic galactic arms.
- **Radial Clustering & Screen-Space Click**: Heavy concentration at the nucleus nexus (`#NEXUS-0`). Clicking anywhere selects the nearest data star and opens its provenance inspector.
- **Domain Filter Rail**: Filter between `BIBLE`, `ASTRONOMY`, `MEDICAL`, `SURVIVAL`, `LITERATURE`, and `COGNITIVE` domains.

### 2. `02 CONSTELLATIONS` (Celestial Astrometry Dome, 3D Sphere & Catalog)
- **Live Celestial Dome (Default)**: 100% offline closed-form astrometry engine computes real-time horizontal coordinates ($Alt/Az$) from local Julian Day, Greenwich Mean Sidereal Time (GMST), and Local Sidereal Time (LST).
- **Interactive Simulation Controls**: UTC time simulation (`-1H`, `+1H`, `NOW`), GPS city presets (Tokyo, Jerusalem, Alexandria, London, New York, Honolulu, Sydney, Cape Town, Cairo), and manual coordinate overrides.
- **View Modes**: Switch seamlessly via the top workspace HUD dropdown between **Live Dome**, **3D Sphere Dome**, and **Poster Catalog Grid**.

### 3. `03 LIBRARIAN` (Deep Reading Deck & Knowledge Hierarchy)
- **Category Hierarchy Explorer**: Structured knowledge tree drilldown: `Theme > Language > Book/Document > Chapter > Section/Passage` with interactive breadcrumbs and direct chapter pagination.
- **Grounded AI Assistant**: Real-time token streaming chat grounded strictly in verified passages. Returns exact citation pills and refuses ungrounded/out-of-domain queries.
- **Spotlight Search (⌘K)**: Global FTS5 BM25 search across all indexed chunks with highlighted context snippets.

### 4. `04 MAP` (Offline Vector Cartography)
- **MapLibre GL & PMTiles**: Zero-cloud HTTP Range byte-serving for single-file `.pmtiles` vector archives.
- **Schema-Aware Styling**: Automatically switches between Natural Earth world overview and Protomaps-v4 street-level vector layers (roads, buildings, waterways, boundaries, places).
- **In-App Downloader**: Curated 62-country downloader running as an atomic asynchronous background job on the server (`POST /api/maps/fetch`) with live progress logs.

---

## Technology Stack

- **Backend Daemon**: Rust 2021 edition (`crates/core`, `crates/server`, `crates/cli`).
- **Web API Layer**: Axum 0.8 with tower middleware, SSE (Server-Sent Events), and custom HTTP `206 Partial Content` Range streamer.
- **Embedded Database**: SQLite 3 with FTS5 BM25 full-text indexing, WAL journaling, and SHA-256 idempotency logs.
- **Astrometry Engine**: Pure Rust closed-form astronomical algorithms (Howard Hinnant civil calendar, GMST polynomial, spherical trigonometry).
- **Frontend Engine**: Vanilla JavaScript (ES modules) + Three.js 3D WebGL + MapLibre GL 3.6.2 + PMTiles protocol adapter.
- **Typography & Assets**: Complete 256-range Klokantech Noto Sans CJK glyph PBFs (30.4 MB) served offline with strict sub-service routing.

---

## Hardware Profiles

| Profile | Hardware | Features Active | Memory Footprint |
|---|---|---|---|
| `tiny` | Raspberry Pi 4 4GB, Pi Zero 2 W | Reader, FTS5 Search, 2D/3D Galaxy, Constellations & Dome, Maps, Notes (No LLM) | Server < 150 MB |
| `standard` | Orange Pi 5 8GB, Pi 5 8GB | Everything + Ministral 3B Q4 via Ollama (2–4k context) | Server < 200 MB, LLM ~2.5 GB |
| `full` | 16GB+ / Desktop | Everything + larger context & models | Unconstrained |

---

## Quickstart

### 1. Build & Install

```bash
./install.sh
```

Installs `librarian` (and symlinked `mazzaroth`) into `~/.local/bin/`.

### 2. Run Diagnostics & Ingest Starter Corpus

```bash
# Run system diagnostics
librarian doctor

# Ingest verified public-domain starter corpus
librarian ingest content/pd-demo/
```

### 3. Launch Server

```bash
librarian serve
```

Opens **http://127.0.0.1:8080** in your browser.

---

## CLI Reference

```
Usage: librarian [COMMAND] [OPTIONS]

Commands:
  serve     Start the offline librarian server (default)
  ingest    Ingest a directory of JSON books with mandatory provenance validation
  doctor    Run diagnostic checks on RAM, disk, SQLite FTS5 integrity, and LLM reachability
  reindex   Optimize SQLite database and rebuild FTS5 index
  maps      Offline map regions: fetch <MIN_LON,MIN_LAT,MAX_LON,MAX_LAT> street-level packs | list
  help      Print this message or the help of the given subcommand(s)

Options:
  -b, --bind <BIND>                  [default: 0.0.0.0:8080]
  -d, --db <DB>                      [default: data/mazzaroth.db]
      --profile <PROFILE>            [default: standard] [possible values: tiny, standard, full]
      --content-dir <CONTENT_DIR>    [default: content]
      --maps-dir <MAPS_DIR>          [default: maps]
      --llm-endpoint <LLM_ENDPOINT>  [default: http://127.0.0.1:11434]
      --llm-model <LLM_MODEL>        [default: ministral-3b]
```

### Offline Map Regions (street-level packs)

```bash
# Fetch a bbox pack from the Protomaps planet build (internet once; served offline afterwards)
mazzaroth maps fetch 18.34,-33.96,18.49,-33.86 --name cape-town        # city ≈ z15
mazzaroth maps fetch 5.75,49.44,6.53,50.18 --name luxembourg --maxzoom 14  # country tier
mazzaroth maps list                                                     # installed regions
```

Packs land in `maps/*.pmtiles`, appear instantly in `/api/maps`, and show up in the
Map view's **INSTALLED REGIONS** panel — click a chip to switch (schema-aware:
overview `countries`/`labels` vs street `roads`/`buildings`/`places`). The official
`pmtiles` extract binary auto-downloads to `data/bin/` on first fetch.

**In the UI (no CLI needed):** the map tool rail has **↓ DOWNLOAD** (pick one of
62 countries → *FETCH & INSTALL* → live progress → the pack auto-opens when
done) and **＋ / − zoom** buttons. Every non-world chip has a **✕** to delete its
pack (confirm dialog; `world.pmtiles` is protected). Fetches run as one
background job on the server — reload/close the page freely, status streams via
`GET /api/maps/fetch`.
Data © OpenStreetMap contributors · ODbL (Protomaps builds).

---

## API Endpoints

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/health` | Healthcheck (`OK`) |
| `GET` | `/api/categories` | List document categories |
| `GET` | `/api/tree` | Full structured category $\to$ language $\to$ book $\to$ chapter hierarchy |
| `GET` | `/api/documents` | List all document summaries |
| `GET` | `/api/read/{doc_id}` | Retrieve full structured document with provenance |
| `GET` | `/api/search?q={query}` | FTS5 BM25 search with highlighted snippets |
| `GET` | `/api/sky` | Real-time astronomical celestial dome projection (Alt/Az, LST, stars, lines) |
| `POST` | `/api/ask` | Grounded AI Librarian SSE streaming chat |
| `GET` | `/api/galaxy` | 3D/2D knowledge graph nodes and link edges |
| `GET` | `/api/sections/constellations` | 32 classical asterism and zodiac catalog |
| `GET` | `/api/notes` | List field notes |
| `POST` | `/api/notes` | Create field note |
| `GET` | `/api/maps` | List available `.pmtiles` map regions |
| `GET` | `/api/maps/fetch` | Current background fetch job status + log tail |
| `POST` | `/api/maps/fetch` | Start a bbox pack fetch (`{"bbox":[w,s,e,n],"name":"…","maxzoom":14}`) |
| `DELETE` | `/api/maps/{filename}` | Delete an installed pack (`world.pmtiles` protected) |
| `GET` | `/maps/{filename}` | HTTP Range byte-serving for PMTiles |

