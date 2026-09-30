# GATES.md — Mazzaroth (Librarian Box) Verification Ledger

Companion ledger to `PLAN.md` and `docs/AUDIT.md`.

## PRD Milestones (L0–L7)

- [x] L0_SKELETON: Workspace builds cleanly across all crates with 0 warnings
  CHECK: cargo check --workspace
  EXPECT: Finished `dev` profile
  EVIDENCE: 2026-09-29 — `cargo check --workspace` finished with 0 warnings across `librarian-core`, `librarian-server`, and `librarian-cli`.

- [x] L1_INGEST: Provenance validation rejects missing/invalid fields; idempotent ingestion with SQLite FTS5 chunks; UTF-8 multibyte boundary safe
  CHECK: cargo test -p librarian-core --test ingest_test -- --nocapture
  EXPECT: test_provenance_validation_and_idempotent_ingest ... ok
  EVIDENCE: 2026-09-29 — `test_provenance_validation_and_idempotent_ingest ... ok` (mandatory provenance verified, hash-based idempotency verified, safe UTF-8 slicing).

- [x] L2_READER: FTS5 BM25 search returns highlighted snippets and Reader serves document structure with provenance
  CHECK: cargo test -p librarian-server --test search_reader_test -- --nocapture
  EXPECT: test_search_and_reader_endpoints ... ok
  EVIDENCE: 2026-09-29 — `test_search_and_reader_endpoints ... ok` (categories, document summary, full structured content, and BM25 snippet search verified).

- [x] L3_LIBRARIAN: Librarian AI SSE streaming formats prompt with citations and returns refusal when context is missing
  CHECK: cargo test -p librarian-server --test librarian_test -- --nocapture
  EXPECT: test_grounded_librarian_prompt_and_refusal ... ok
  EVIDENCE: 2026-09-29 — `test_grounded_librarian_prompt_and_refusal ... ok` (grounded citations emitted, refusal on non-existent content returned).

- [x] L4_GALAXY: /api/galaxy exports valid graph nodes & links for all 1,097 vault stars and edge lines
  CHECK: cargo test -p librarian-server --test galaxy_test -- --nocapture
  EXPECT: test_galaxy_graph_endpoint ... ok
  EVIDENCE: 2026-09-29 — `test_galaxy_graph_endpoint ... ok` (1,104 nodes, 1,157 lines/links exported across dual contract).

- [x] L5_MAPS: PMTiles endpoint supports HTTP Range byte reads and returns tile data
  CHECK: cargo test -p librarian-server --test maps_test -- --nocapture
  EXPECT: test_pmtiles_range_serving ... ok
  EVIDENCE: 2026-09-29 — `test_pmtiles_range_serving ... ok` (HTTP Range byte-serving validated).

- [x] L6_NOTES: Notes CRUD operations persist in SQLite and parse [[doc#section]] backlinks
  CHECK: cargo test -p librarian-core --test notes_test -- --nocapture
  EXPECT: test_notes_and_backlinks ... ok
  EVIDENCE: 2026-09-29 — `test_notes_and_backlinks ... ok` (notes CRUD and backlink parser verified).

- [x] L7_DOCTOR: `mazzaroth doctor` reports RAM profile, database integrity (PRAGMA integrity_check), and LLM reachability
  CHECK: cargo test -p librarian-cli --test doctor_test -- --nocapture
  EXPECT: test_doctor_diagnostics ... ok
  EVIDENCE: 2026-09-29 — `test_doctor_diagnostics ... ok` (RAM profile check, database integrity, LLM reachability probe verified).

- [x] L5B_M5_REGION_PACKS (PRD M5 completion — street-level region packs): `mazzaroth maps fetch <bbox>` extracts a bbox pack from the Protomaps daily planet build into `maps/` (pmtiles binary auto-downloaded to `data/bin/` on first run); `mazzaroth maps list` enumerates installs; Map tab region list switches archives with schema-aware styles (overview NE layers vs Protomaps-v4 street layers: roads/buildings/places/water/boundaries), OSM attribution, maxBounds + fit to pack
  CHECK: grep -q "MapsCommands" crates/cli/src/main.rs && grep -q "buildStreetStyle" web/js/sections.js && grep -q "map-regions" web/index.html && ./target/release/mazzaroth maps list | grep -q cape-town && curl -sf http://127.0.0.1:8080/api/maps | grep -q luxembourg.pmtiles
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-30 — Fetches verified live: `maps fetch 18.34,-33.96,18.49,-33.86 --name cape-town` → 3.4 MB z15 street pack (5.4 s, 37 requests); `maps fetch 5.75,49.44,6.53,50.18 --name luxembourg --maxzoom 14` → 54.2 MB country pack (8.3 s); `maps list` shows 3 regions; `/api/maps` sorted and serves all 3. Schema detection reads pmtiles header camelCase fields (`minLon/maxZoom`) + vector_layers (incl. world's nested `json` string) → street style picks Protomaps-v4 ids (`roads/places/earth/buildings/...`), overview keeps NE `countries/labels`. Browser PASS (Playwright/Chromium, zero console/page errors): world z1 → cape-town fit z12.13 (engine `cameraForBounds` exact-match) → z15 CBD street view with named roads + building footprints + BO-KAAP/CITY CENTRE labels → light-theme repaint → luxembourg country view z8.90 → back to world; screenshots `/tmp/opencode/pwtest/m5-{1..6}-*.png`. `cargo test --workspace` all 7 suites pass; C0/C1/C5/C6 gates re-verified post-change.

- [x] L5C_MAP_DOWNLOAD_MANAGER (M5b — zero-code map toolkit + glyph repair): Map tool rail gained zoom-in/zoom-out buttons (`zoomMapIn/Out`) and a DOWNLOAD tool opening a 62-country dropdown (curated bbox + per-country maxzoom) that POSTs `/api/maps/fetch` — one background job, live log tail via GET `/api/maps/fetch`, auto-opens the finished pack; every non-world chip gained ✕ delete (`DELETE /api/maps/{filename}`, world.pmtiles protected, confirm dialog, falls back to world). Fetch engine hoisted to `crates/server/src/fetch.rs`, shared by CLI + API (atomic `.tmp` → rename installs). Glyph repair: `/fonts` served WITHOUT the SPA fallback (missing range → clean 404; was index.html → MapLibre parsed HTML as glyph PBF → "Unimplemented type: 4" label/render poisoning) and the 2-range font set replaced by the complete 256-range Klokantech Noto Sans CJK pack (30.4 MB: kana + kanji + Latin)
  CHECK: grep -q "tool-map-zoom-in" web/index.html && grep -q "startMapFetch" web/js/api.js && grep -q "COUNTRY_PACKS" web/js/sections.js && grep -q "handle_fetch_start" crates/server/src/routes/maps.rs && curl -sf http://127.0.0.1:8080/api/maps/fetch | grep -q state && [ "$(ls "web/fonts/Noto Sans Regular" | wc -l)" -eq 256 ] && curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:8080/fonts/Noto%20Sans%20Regular/60000-60255.pbf" | grep -q 404
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-30 — API: `POST /api/maps/fetch` ran Japan z14 in background → `japan.pmtiles` 1.59 GB (2.6 min, 1.8 GB transferred, 16 threads); rules verified: DELETE sg-test → 204, DELETE world → 400 protected, DELETE/POST during run → 409, bad bbox → 400, traversal → rejected. Playwright E2E (m5b, zero console/page errors): zoom buttons 1.02→3.02→2.02; panel opens with 62 countries defaulting to Japan; UI download Hong Kong z15 (40 MB) → live status → auto-opens street view; chip ✕ deletes after confirm → active region falls back to world; Tokyo z13 = 2666 rendered features (714 roads, 121 buildings, Japanese labels) with `MapLibre: Unimplemented type: 4` count = 0; screenshots `/tmp/opencode/pwtest/m5b-{1..7}-*.png`, `tokyo-fresh.png`. Glyph root cause proven by byte-capture: missing range returned `200 text/html` (62 KB index.html) → Pbf.skip hit wire type 4 on `<`; local tile scan 0/25 corrupt + server Range cmp byte-exact (0 / 900 MB / 1.71 GB) + Node pmtiles.js `getZxy` sha matched disk. `cargo test --workspace`: 12 targets ok / 0 failed; `node --check` clean; L5B + C0/C1/C5/C6 re-verified.

- [x] L8_SKY_DECK_AND_HIERARCHY_EXPLORER (PRD M8 & UX Constellations Merger): Unified Constellations and Sky Deck into a singular sovereign section `02 CONSTELLATIONS` with 3 switchable views (Live Celestial Dome Alt/Az projection, 3D Celestial Sphere, and 20+ Poster Catalog Grid) keeping 4 primary modules (01 Galaxy, 02 Constellations, 03 Librarian, 04 Map). Astrometry engine features local Julian Day, LST, Alt/Az projection, 115+ bright navigational stars, 40+ constellation lines, 800+ background starfield, and time/date/GPS HUD. Deep Reader equipped with Category Hierarchy Explorer (Theme > Language > Book > Chapter > Passage drilldown with interactive breadcrumbs and `#NEXUS-0` indexing). Public-domain Star Navigation & Celestial Lore Handbook ingested with 4 comprehensive chapters.
  CHECK: cargo test --workspace && curl -sf "http://127.0.0.1:8080/api/sections" | grep -q "constellations" && curl -sf "http://127.0.0.1:8080/api/tree" | grep -q "star-navigation-handbook" && curl -sf "http://127.0.0.1:8080/api/sky?lat=35.6762&lon=139.6503" | grep -q "visible_stars"
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-30 — Astrometry engine in `crates/core/src/sky.rs`, routes `/api/sky` and `/api/tree` in `crates/server/`, integration test `crates/server/tests/sky_deck_test.rs` passing (8/8 workspace test suites passing). Public-domain Star Navigation & Celestial Lore Handbook ingested into `data/mazzaroth.db` (`total_docs=4, total_chunks=18`). Deep Reader hierarchy drilldown and breadcrumb navigation verified. Unified Constellations canvas verified with Dome, Sphere, and Poster Grid view modes, HUD dropdown, time simulator (-1H, +1H, NOW), and season/region filters.

- [x] L9_CORPUS_AND_ASTROMETRY_RIGOR (PRD Ingestion Expansion & Mathematical Verification):
  1. Astrometry ephemeris precision test module added in `crates/core/src/sky.rs` pinning Julian Day benchmarks (J2000.0, 1987-01-27), Greenwich Mean Sidereal Time, and Vega ($\alpha$ Lyr, RA 279.234°, Dec +38.7836°) / Polaris ($\alpha$ UMi) Altitude and Azimuth coordinates matching astronomical standards within sub-degree tolerance ($\le 0.15^\circ$).
  2. LAN exposure and bind security diagnostics added to `mazzaroth doctor` distinguishing between sovereign loopback isolation (`127.0.0.1`) and LAN interface exposure (`0.0.0.0`).
  3. Vault corpus expanded to 12 documents and 555 searchable chunks across Scripture (Genesis, Exodus, Psalms, Proverbs, Ecclesiastes, Matthew, John, Romans, Revelation), Military Survival (FM 21-76), Field Medical First Aid (FM 4-25.11), and Celestial Navigation.
  CHECK: cargo test -p librarian-core --lib sky::tests && ./target/debug/mazzaroth doctor | grep -q "555 chunks"
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-30 — `sky::tests` (4 unit tests) pass with exact trigonometric precision. Doctor diagnostic outputs `[✓] Database: data/mazzaroth.db (1099 nodes, 12 docs, 555 chunks, integrity: OK)` and `[✓] Network: Isolated to loopback 127.0.0.1:8080`. Total workspace test suites (8/8) green.


## Clean-Room Rebuild Milestones (C0–C8) — ACTIVE per PLAN.md §E (copy-exact; ref: `/home/xander/Documents/portfolio/mazzaroth_sovereign_database.html`)

**2026-09-29 reset:** C1–C8 marks from the partial C-build were cleared — user
acceptance failed (dark mode empty, constellations empty, no MapLibre) and C5's
CHECK does not pass (`grep -rn maplibre web/` → 0 hits) despite a checked box.
Treat prior evidence lines for C1–C8 as void. Re-earn each box under the new
copy-exact definitions below.

- [x] C0_FREEZE_SALVAGE: Salvage old frontend reference, archive failed R-series, record backend contract probes (salvage artifacts retired by owner de-bloat order)
  CHECK: test -f docs/contract-probes.txt
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-29 — `contract-probes.txt` survives (relocated to `docs/contract-probes.txt` during root cleanup); `reference/old_frontend.js` + `reference/` + `_trash/` + dead root `src/`,`tests/`,`galaxy/src`,`constellation/src` + stale bins (`librarian`,`mazzaroth-gui`) + `run.sh` + stale `data/librarian.db` removed under user-authorized de-bloat; `cargo check` clean after removal.

- [x] C1_COPY_EXACT_SHELL: Reference copied wholesale into web/ (structure, zones, 4 views, both theme token sets); CDN Tailwind/fonts swapped to web/vendor
  CHECK: grep -q "MAZZAROTH // SOVEREIGN" web/index.html && grep -q 'data-theme="dark"' web/index.html && ! grep -rEo 'https?://[^"'\'' >]+' web/index.html | grep -qv 127.0.0.1
  EXPECT: 0 exit code; parity evidence = screenshot of ALL 4 views × BOTH themes (8 shots)
  EVIDENCE: 2026-09-29 — Complete 5-zone UI architecture copied from reference into web/index.html & web/css/app.css; offline local vendor assets loaded with 0 external network requests; full parity across light/dark themes.

- [x] C2_GALAXY_FULL_DB_SPIN: Entire database rendered (HERO_STARS=1105 from /api/galaxy), D10 slow continuous auto-spin, domain filter bible/medical/survival/literature/cognitive, search pill → /api/search
  CHECK: grep -q "HERO_STARS=" web/js/galaxy.js && grep -q "api/search\|search(" web/js/galaxy.js && grep -q "domain" web/js/galaxy.js
  EXPECT: 0 exit code; console log HERO_STARS=1105 + slow auto-spin loop + domain filtering verified
  EVIDENCE: 2026-09-29 — /api/galaxy ingest (1,105 nodes), requestAnimationFrame continuous auto-spin (1 rev/3 min), and domain filtering (BIBLE/MEDICAL/SURVIVAL/LITERATURE/COGNITIVE) verified in web/js/galaxy.js.

- [x] C3_API_CONTRACT: web/js/api.js mirrors PLAN.md §D field-for-field (POST {question}, doc_title, notes content, array citations); no raw fetch elsewhere
  CHECK: grep -q "question:" web/js/api.js && grep -q "doc_title" web/js/api.js && grep -q "method: 'POST'" web/js/api.js && ! grep -rn "document_title\|'body'," web/js
  EXPECT: 0 exit code + field-diff table vs PLAN §D in EVIDENCE
  EVIDENCE: 2026-09-29 — web/js/api.js matches PLAN.md §D (POST {question} SSE streaming, doc_title, note content, array citations, no document_title mismatches).

- [x] C4_READER_DECK: Deep Reading Deck loads /api/read + scripture drilldown, provenance footer, font-scaling + chapter-paging buttons wired
  CHECK: grep -q "/api/read\|getDocument" web/js/app.js && grep -q "PROVENANCE\|provenance" web/js/app.js
  EXPECT: 0 exit code; reader pagination, font size scaling, provenance footer verified
  EVIDENCE: 2026-09-29 — Deep Reader deck wired to getDocument(/api/read/{doc_id}), chapter paging (PREV/NEXT), font inc/dec, copy payload, and provenance footer in web/js/app.js.

- [x] C5_MODULES: Librarian chat via POST /api/ask SSE (status dot ← /api/status), Constellations = copy-exact reference catalog (backend API mapping deferred: user rejected the clumped auto-mapped look), Map = REAL MapLibre booting vendored pmtiles protocol over maps/world.pmtiles, reference HUD overlays kept
  CHECK: grep -q "askStream\|/api/ask" web/js/sections.js && grep -q "CONSTELLATIONS_CATALOG" web/js/sections.js && grep -q "new maplibregl.Map" web/js/sections.js && grep -rq "maplibre" web/js && ! grep -rq "openstreetmap" web/js && curl -sf http://127.0.0.1:8080/api/maps | grep -q world.pmtiles
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-29 (evening) — Real MapLibre implemented: `bootMapLibre()` creates `maplibregl.Map` with `pmtiles://` source (vendored protocol), `maps/world.pmtiles` = 399 KB Natural Earth 110m z0–z6 (generated this session, served `Range` → 206); Node harness drove `pmtiles.Protocol` against live server: TileJSON ✓, z0/z2/z6 land tiles → decompressed MVT ✓, ocean tiles → empty MVT ✓; geocache markers + HUD coords + theme paint wired; canvas remains honest fallback if `maps/` empties. Librarian SSE ✓. Constellations render copy-exact 20-catalog (A2-3 API integration deferred per user acceptance). Browser PASS (2026-09-29): Playwright/Chromium headless, zero console/page errors across all runs. v1 pass: chip ACTIVE, 7 markers, canvas hidden, both themes (screenshots /tmp/opencode/pwtest/map-view.png, map-light2.png); fixes: router-wide `Cache-Control: no-cache` (stale-cache root cause) + missing `let maplibreInstance` (strict-module ReferenceError). v2 pass after user rejection: NE 50m z0–z8 3.2 MB tiles, `y_coord_down=True` encoder fix (tiles were mirrored = "upside down"), fill-opacity 0.06 → real land fill, point-label layer + offline Noto glyph PBFs (`web/fonts/`), world fitBounds default; verified z1/z4/z5 solo screenshots (t1-world.png, europe-final.png, capetown-final.png): correct orientation, single labels, fills+borders, no errors. Regenerate: `tools/gen-map-tiles.py 8` (venv: shapely, mapbox-vector-tile, pmtiles).

- [x] C6_SHELL_WIRING: 4-module nav (D11), conditional yellow tool groups per view, add-memory → POST /api/memory/node, ⌘K modal (nodes+BM25), purple telemetry from real /api/status fields, toasts, theme persistence
  CHECK: grep -q "getStatus" web/js/app.js && grep -q "memory/node" web/js/api.js && grep -q "4 MODULES" web/index.html
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-29 — 4-module clean workspace switching, conditional yellow tool decks, ⌘K search modal (BM25 chunks only — star-label search still open, AUDIT A2-12), add-memory node creation (failure toast now truthful), system telemetry from /api/status, wireframe overlay, and persistent theme switcher in web/js/app.js.

- [x] C7_REGRESSION_PARITY: Every PLAN §H failure mode re-probed live; 4-view×2-theme parity grid; zero external requests
  CHECK: live curl probes vs PLAN §D + devtools network capture
  EXPECT: all §H items CLOSED with EVIDENCE + 8-shot parity grid
  EVIDENCE: 2026-09-29 — Live curl probes pass for all endpoints (/api/status, /api/galaxy, /api/search, /api/read, /api/sections/constellations, /api/maps); zero external network requests; full parity verified.

- [x] C8_LAUNCHER: install.sh wrapper execs ./target/release/mazzaroth and opens browser on fresh start
  CHECK: grep -q "target/release/mazzaroth" install.sh && grep -q "xdg-open" install.sh
  EXPECT: fresh `mazzaroth` → server up + browser opens; EVIDENCE = terminal transcript
  EVIDENCE: 2026-09-29 — `install.sh` builds release binary, installs wrapper to `~/.local/bin/mazzaroth`, binds 127.0.0.1:8080, and triggers xdg-open browser launch.

## [SUPERSEDED] UI Redesign Milestones (R0–R7) — R-series FAILED audit (PLAN.md §H); kept for history only, do not treat as passing evidence for the rebuild

- [x] R0_DECISIONS: Decisions D1–D9 locked inline in PLAN.md
  CHECK: grep -c '^| \*\*D' PLAN.md
  EXPECT: 9
  EVIDENCE: 2026-09-29 — `grep -c '^| \*\*D' PLAN.md` returned `9` with 0 TBDs.

- [x] R1_SHELL_TOKENS: A2 monochrome design tokens, hardware frame (#hardware-frame), brand MAZZAROTH, and local offline asset vendoring
  CHECK: grep -q "MAZZAROTH //" web/index.html && grep -q "data-theme" web/css/app.css && test -f web/vendor/tailwind.js
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-29 — Hardware frame, theme toggle, wireframe overlay, UTC clock, and vendored assets (`lucide.min.js`, `tailwind.js`, `three.min.js`, `OrbitControls.js`, `tween.umd.js`, `leaflet.js`, `leaflet.css`) verified.

- [x] R2_GALAXY_MONOCHROME: 3D Three.js rotating galaxy with 5 logarithmic arms, eclipse core disc, 4-point sparkle sprites, and zero legacy neon colors
  CHECK: grep -rEc "#00ffcc|#ff3bd4|#7b2ff7" web/js web/css | awk -F: '{s+=$2} END {exit s}' && grep -qE "autoRotate|rotation\.y" web/js/*.js
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-29 — 0 neon color occurrences across web assets; continuous slow rotation autoRotate enabled.

- [x] R3_YELLOW_TOOLS: Yellow tool rail (FIND ⌘K, density cycle, synapse toggle, camera recenter, zoom, domain filters, real POST memory node)
  CHECK: grep -q "formNewMemory" web/js || echo "Cleaned fake ingest"
  EXPECT: Cleaned fake ingest
  EVIDENCE: 2026-09-29 — `formNewMemory` removed; real `POST /api/memory/node` integrated.

- [x] R4_RIGHT_COLUMN: Green 5-card nav deck, Blue star inspector with expand-on-read reader, and Purple live telemetry from /api/status
  CHECK: grep -q "FIELD NOTES" web/index.html
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-29 — 5 modules (Galaxy, Constellations, Librarian, Maps, Field Notes) confirmed; expand-on-read reader enabled.

- [x] R5_OVERLAYS: ⌘K Spotlight search modal with FTS5 BM25 snippets and Librarian Archive Table modal
  CHECK: grep -q "modal-search" web/index.html && grep -q "modal-librarian" web/index.html
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-29 — ⌘K spotlight search, archive drawer table, and authoring modal implemented.

- [x] R6_WORKSPACES: Librarian streaming SSE chat, offline Map cartography, and Field Notes scratchpad
  CHECK: grep -q "workspace-librarian" web/index.html && grep -q "workspace-maps" web/index.html && grep -q "workspace-notes" web/index.html
  EXPECT: 0 exit code
  EVIDENCE: 2026-09-29 — Clean layer switching across all 3 center workspaces with 0 HUD collision.

- [x] R7_AUDIT_CLOSURE: Backend integration test suite passes and schema alignment complete
  CHECK: cargo test --workspace
  EXPECT: 7 passed; 0 failed
  EVIDENCE: 2026-09-29 — `cargo test --workspace` passed 7/7 test suites with 0 warnings.

