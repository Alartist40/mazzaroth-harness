# IMPLEMENT.md — Chronological Implementation Log

## [2026-09-28] M0–M7 Complete Offline Librarian Box Architecture
- **Decision**: Restructured repository into multi-crate Cargo workspace (`crates/core`, `crates/server`, `crates/cli`).
- **Core Domain (`crates/core`)**:
  - Implemented hardware profiles (`tiny`, `standard`, `full`) with memory caps and top-k retrieval bounds.
  - Implemented Content JSON schema with mandatory provenance validation (`source`, `publisher`, `license` [public-domain, cc-by, cc-by-sa, cc0, gutenberg, personal-use-only], `retrieved_date`).
  - Implemented SQLite database layer with tables `documents`, `chunks`, `chunks_fts` (FTS5 BM25 search index + triggers), `notes`, `ingest_log` (SHA-256 idempotency).
  - Implemented PMTiles header and byte-range reading engine.
- **Server (`crates/server`)**:
  - Built Axum server with routes `/health`, `/api/documents`, `/api/categories`, `/api/read/:doc_id`, `/api/search`, `/api/ask` (SSE streaming), `/api/galaxy`, `/api/notes`, `/api/maps`, `/maps/:filename` (HTTP Range).
  - Implemented grounded AI Librarian prompt assembly with citation emission, out-of-context refusal (*"I don't have that in the library"*), and graceful offline degradation.
- **CLI (`crates/cli`)**:
  - Built `librarian serve`, `librarian ingest <dir>`, `librarian doctor` (RAM, disk, SQLite FTS5, and LLM diagnostics), and `librarian reindex`.
- **UI (`ui/src/`)**:
  - Built responsive multi-view web interface: 4-level Reader with Provenance Footers, Search with highlighted snippets, Librarian AI chat with streaming tokens and citations, 3D Three.js particle galaxy with 2D Canvas fallback, Offline Maps viewer, and Field Notes markdown editor with `[[doc#section]]` backlinks.
- **Verification**: All 8 PRD gates (`L0_SKELETON` through `L7_DOCTOR`) passing.

## [2026-09-29] Clean-Room Rebuild: C0 Salvage Archive & C1 Copy-Exact Shell
- **C0 Execution**: Archived rejected partial C-build (`web/index.html`, `web/css/app.css`, `web/js/*`) into `_trash/c-partial/`.
- **C1 Execution**: Copied sovereign reference structure wholesale from `/home/xander/Documents/portfolio/mazzaroth_sovereign_database.html` into `web/index.html` and `web/css/app.css`.
- **Token Parity & Offline Compliance**: Embedded light (`:root`) and dark (`[data-theme="dark"]`) design tokens, 5-zone UI grid, 4 view panels, toolbars, modal search, toasts, and wireframe guide. Replaced CDN Tailwind and Google Fonts with vendored assets (`web/vendor/tailwind.js`, system mono/sans stacks) with zero external network dependencies.
- **Gates Verified**: `C0_FREEZE_SALVAGE` and `C1_COPY_EXACT_SHELL` passing.

## [2026-09-29] Clean-Room Rebuild: C2–C6 Implementation
- **C2 Execution**: Built `web/js/galaxy.js` rendering all 1,105 nodes from `/api/galaxy`, continuous slow rotation auto-spin (1 revolution per 3 mins), and domain filtering (`BIBLE`, `MEDICAL`, `SURVIVAL`, `LITERATURE`, `COGNITIVE`, `ALL`).
- **C3 Execution**: Built `web/js/api.js` contract layer mirroring PLAN.md §D (SSE streaming via `POST /api/ask`, `doc_title`, `notes` content, array citations).
- **C4 Execution**: Implemented Deep Reading Deck in `web/js/app.js` supporting structured document inspection (`/api/read/{doc_id}`), chapter pagination, font size scaling, copy payload, and provenance footer.
- **C5 Execution**: Built `web/js/sections.js` for Constellations (32-asterism catalog + season/position filters), Librarian (real-time token streaming + citation pills), and Map (MapLibre offline PMTiles + coordinate HUD).
- **C6 Execution**: Wired shell in `web/js/app.js` with 4-module navigation, conditional yellow tool decks, persistent light/dark theme switcher, ⌘K search modal, add-memory node creation, and telemetry specs from `/api/status`.

## [2026-09-29] Aesthetic & 3D/2.5D Upgrade (Galaxy, Constellations, MapLibre)
- **Galaxy 2.5D Transformation**: Built 3D Three.js scene with OrbitControls on `galaxy-canvas`. Ingested 1,105 nodes as camera-facing billboarded sprites (2.5D effect that never flattens when orbiting).
- **Aesthetic De-cluttering**: Completely eliminated pervasive text label rendering from the canvas. Replaced with dynamic floating hover tooltip (`#galaxy-tooltip`) and selection highlight.
- **Radial Concentration**: Adjusted particle and star distributions with cubic/power-law bias (`distBias = Math.pow(..., 3.6)`), heavily clustering stars near the core nexus and dispersing gently along outer spiral arms.
- **Click-Anywhere Nearest Star**: Integrated screen-space projection (`findClosestStarToScreen`) that finds and selects the nearest database star to the click coordinate.
- **Constellations Upgrade**: Rebuilt Constellations into 3D celestial asterisms with glowing star vertices, golden Zodiac vector lines, and azure Classical lines.
- **Offline MapLibre Integration**: Vendored `maplibre-gl.js` and `maplibre-gl.css` offline into `web/vendor/`; initialized MapLibre GL instance in `#maplibre-container` with coordinate HUD and geocache markers.

## [2026-09-29] Reference-Exact Refinements & Galaxy Visual Polish
- **Core Center Refinement**: Removed orbital ring mesh entirely from galaxy core nucleus; fixed volumetric spiral math with smooth continuous radial noise to completely eliminate flat vertical paper-like strand artifacts in the center.
- **Dense Luminous Disc**: Increased background particle count to 16,000 with scaled center-to-edge point sizes (3.4px to 4.8px) for a rich, vibrant cosmic visual with strong core concentration.
- **Zoom In/Out Inversion Fix**: Corrected zoom direction so `tool-zoom-in` smoothly zooms in closer and `tool-zoom-out` zooms out.
- **Reference-Exact Constellations**: Integrated the full 20-constellation catalog, poster grid reticles, dashed connecting vector lines, double-circle bright star vertices, and celestial sphere dome mode copied directly from `/home/xander/Documents/portfolio/mazzaroth_sovereign_database.html`.
- **Reference-Exact Map**: Integrated the coordinate graticule grid, vector continent blocks, double-concentric target geocache pins, coordinate hover HUD, and pan/zoom handlers copied directly from `/home/xander/Documents/portfolio/mazzaroth_sovereign_database.html`.
