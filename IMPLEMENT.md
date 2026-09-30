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

## [2026-09-30] Sky Deck Live Astrometry Dome & Category Hierarchy Explorer
- **05 SKY DECK Astrometry Engine (`crates/core/src/sky.rs`)**:
  - Implemented 100% offline closed-form astronomical equations: Julian Day Calculation (Howard Hinnant civil calendar conversion), Greenwich Mean Sidereal Time (GMST polynomial), Local Sidereal Time (LST = GMST + Longitude), Hour Angle ($H = LST - RA$), and Horizontal Alt/Az Spherical Trigonometry ($\sin(Alt) = \sin\delta\sin\phi + \cos\delta\cos\phi\cos H$, $\cos(Az) = (\sin\delta - \sin\phi\sin Alt)/(\cos\phi\cos Alt)$).
  - Built comprehensive star catalog with 115+ bright navigational stars across 30+ constellations, 40+ vector stick figure lines, and 800+ background starfield points.
  - Added dynamic centroid labeling and Zenith/Nadir circular stereographic dome projections.
- **Backend Sky & Tree Endpoints (`crates/server/src/routes/`)**:
  - Registered `GET /api/sky?lat=...&lon=...&time=...` returning real-time dome-projected stars, azimuth/elevation coordinates, vector lines, and visibility filters.
  - Registered `GET /api/tree` returning the complete hierarchically indexed knowledge categories, languages, documents, chapters, and section IDs.
  - Created automated test suite `crates/server/tests/sky_deck_test.rs` covering coordinate calculations, vector lines, and tree hierarchy traversal.
- **Star Navigation Handbook Ingestion**:
  - Authored and validated public-domain `content/pd-demo/star-navigation-handbook.json` with 4 detailed technical chapters (Introduction to Celestial Navigation, Bright Star Identification & Sight Reduction, Polar Alignment & Latitude Determination, Constellation Lore & Astrometry Tables).
  - Ingested into `data/mazzaroth.db` SQLite FTS5 database (`total_docs=4, total_chunks=18`).
- **Interactive Deep Reader Hierarchy Explorer (`web/js/app.js`, `web/css/app.css`)**:
  - Overhauled Deep Reader with an interactive Category Hierarchy Explorer when `#NEXUS-0` is focused.
  - Added visual drilldown: `Theme (Astronomy, Survival, Medical, Scripture, Cognitive) > Language > Document/Book > Chapter > Passage`.
  - Added interactive breadcrumbs (`#reader-breadcrumbs`) with direct jump buttons back to any level in the knowledge tree.
- **Sky Deck UI Canvas & Controls (`web/js/sections.js`, `web/index.html`)**:
  - Added 5th System Section `05 SKY DECK` in Zone 3 nav deck and `#view-sky` canvas in Zone 2.
  - Implemented exact visual design parity with Constellations module: `[3, 3]` dashed vector lines, concentric double-circle halos (`r=6px, lineWidth=0.6`) on bright navigational stars, alt/az coordinate rings, and dark/light token compatibility.
  - Added Time/Date and GPS Lat/Lon HUD controls with live recalculation and star click-to-reader integration.

## [2026-09-30] Constellations & Sky Deck Unification into 4-Module Layout
- **Single Cohesive Section (`02 CONSTELLATIONS`)**:
  - Merged Sky Deck and Constellations into a single section, returning to a clean 4-module system (`01 GALAXY`, `02 CONSTELLATIONS`, `03 LIBRARIAN`, `04 MAP`).
  - Added a top HUD dropdown `#const-mode-select` in the main workspace allowing instant switching between:
    1. `01 // CELESTIAL DOME (LIVE)` (Live astrometry projection with GPS presets and UTC time/date simulation)
    2. `02 // SPHERE DOME (3D)` (3D celestial sphere projection)
    3. `03 // POSTER CATALOG (GRID)` (20+ poster grid catalog with selection reticles)
- **Tool Rail Consolidation (`web/index.html`, `web/js/sections.js`)**:
  - Unified `#tools-constellations` with layout mode switcher, time controls (-1H, +1H, NOW), season cycling, region cycling, vector lines toggle, zoom in/out, and recenter.
  - Time controls and GPS dropdown automatically display in `DOME` mode and gracefully collapse in `POSTER` or `SPHERE` mode.
- **Engine Unification (`web/js/sections.js`, `web/js/app.js`)**:
  - Unified `initConstellations()` to render all three visualization modes onto `constellations-canvas`.
  - Removed redundant `#view-sky` and `#tools-sky` markup and standalone loop blocks.
  - Verified with `node --check` and `cargo test --workspace` (8/8 test suites passing).
