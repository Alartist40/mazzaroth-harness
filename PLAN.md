# PLAN.md — Mazzaroth Frontend Rebuild (Sovereign Reference, copy-exact)

Mode: **CLEAN-ROOM C0–C8 [ACTIVE]** — backend frozen & verified. Supersedes the failed
R-series (§H) and the rejected partial C-build. Companion ledger: `GATES.md`.

## Next action

**Map tab is live & browser-verified** (2026-09-29 evening): headless Chromium
ran the app end-to-end — chip `world.pmtiles ACTIVE (z0–z6 offline)`, 7 markers,
both themes render, zero page errors (screenshots from that run:
`/tmp/opencode/pwtest/map-view.png`, `map-light2.png`). Two defects found &
fixed en route: router-wide `Cache-Control: no-cache` (stale-cache was hiding
the new build from the browser) and a missing `let maplibreInstance` declaration
(strict-module ReferenceError). What is now true, verified this session:

1. **Map = real MapLibre, v2 rebuild (2026-09-29 late)**: user rejected v1
   ("not a map… cut in half and upside down, glitches on zoom, no country
   details"). Root causes found & fixed: (a) `fill-opacity: 0.06` rendered land
   invisible — only hairlines showed; (b) `mapbox_vector_tile.encode` default
   `y_coord_down=False` **mirrored every tile vertically** (the "upside down") —
   fixed with `default_options={"y_coord_down": True}` in `tools/gen-map-tiles.py`;
   (c) old default view cut the Americas (`center [10,25] zoom 1.5`) — now
   `fitBounds` full world; (d) 110m data z6 too coarse (the "glitches"/no
   detail). Rebuilt: Natural Earth **50m**, z0–z8, 3.2 MB / 39,638 tiles,
   polygon + point-label layers; style = distinct land/ocean fills, zoom-scaled
   borders, graticule, country labels; offline glyph fonts (Noto Sans PBFs
   generated from system TTF via fontnik into `web/fonts/`, served by static
   handler; `glyphs:` URL in style). Browser-verified in headless Chromium at
   z1 world / z5 Cape Town / z4 Europe: correct orientation, single labels,
   borders + fills, both themes, zero page errors (screenshots:
   `/tmp/opencode/pwtest/t1-world.png`, `capetown-final.png`, `europe-final.png`).
   PRD M5 world-overview tier (~3 MB) regenerated via `tools/gen-map-tiles.py`.
   `bootMapLibre()` also got a self-reporting status chip (`#maplibre-status`)
   and `window.__maplibre` test hook.

2. **M5 street-level region packs BUILT (2026-09-30, user request)**:
   `mazzaroth maps fetch MIN_LON,MIN_LAT,MAX_LON,MAX_LAT [--name N] [--maxzoom Z]`
   resolves the latest Protomaps daily planet build (`build-metadata.protomaps.dev/builds.json`
   → 138 GB remote archive), auto-downloads the official `pmtiles` binary to
   `data/bin/` on first use, and range-extracts only the bbox tiles (HTTP
   Range — never the whole planet): Cape Town city = 3.4 MB in 5.4 s,
   Luxembourg country (z14) = 54.2 MB in 8.3 s. `mazzaroth maps list` enumerates
   installs; `/api/maps` now sorts by filename. Map tab gained a **region list
   panel** (bottom-right, chips with sizes + `+ mazzaroth maps fetch <bbox>` hint)
   and **schema-aware styles**: header/vector_layers detection (incl. world's
   nested `json` metadata) → street style renders Protomaps-v4 layers
   (earth/water/landuse/buildings/boundaries/roads by kind_detail widths,
   rail/ferry/path dashes, road labels on line placement, locality + neighbourhood
   labels) in the same monochrome-wireframe aesthetic, with `maxBounds` + fit to
   pack bounds, pack maxzoom, OSM/ODbL attribution (offline plain text), and
   light/dark paint swap via `applyMapLibreTheme()` branch. Region switching
   tears down markers + instance and re-mounts; choice persists in localStorage.
   Verified: Playwright full round trip world → cape-town → z15 CBD → theme →
   luxembourg → world, zero console/page errors, engine-exact fits
   (GATES L5B evidence; screenshots /tmp/opencode/pwtest/m5-{1..6}-*.png).
   `maps/*.pmtiles` + `data/bin/` gitignored (regenerable: gen-map-tiles.py /
   maps fetch).
3. **Reader shows real corpus** (was A2-2 P0): `loadNodeIntoReader` maps
   `doc.structure[].sections[].text` → chapters + provenance footer (C4 CHECK ✓).
4. **Root cleanup + layout done (user order 2026-09-29, second pass)**:
   deleted `data/librarian.db` (stale duplicate — live DB is `data/mazzaroth.db`,
   CLI `--db` default `crates/cli/src/main.rs:19`, holds the 2 notes;
   `LibrarianConfig::default()` repointed from `librarian.db` → `mazzaroth.db`),
   `run.sh` (redundant), `reference/` dir; relocated `contract-probes.txt` and
   `AUDIT.md` → `docs/`. Root now: 5 md + install.sh + 8 code/data dirs.
   `AGENTS.md` architecture section rewritten (old text described deleted
   `src/`, `tests/`, `main.js`, `base.css`). GATES C0/C1-R1 CHECKs updated to
   surviving paths; `cargo check` + all 7 test suites + C0/C5/C6/R1 CHECKs +
   browser sanity re-passed after the move.
5. **Remaining OPENs**: A2-12 (⌘K searches chunks only, no star/node labels),
   A2-10 (fonts never load), A2-11 (duplicate theme tokens app.css vs inline),
   A2-6 (galaxy uses synthetic spiral coords, ignores stored x/y/z),
   A2-9 (domain filter colors — owner call), A2-5 (corpus = 3 docs/9 chunks,
   ingest more content), A2-3 (constellation API mapping — deferred by owner).

**Remaining open (not blocking launch):** A2-12 ⌘K searches BM25 only (star
labels not searchable); A2-10 fonts declared but never loaded (system fallback);
A2-11 duplicate theme tokens + dead `--core-*` vars; A2-6 galaxy still ignores
stored `x/y/z` + synthetic `STAR-####` filler on short API; A2-9 domain colors
vs monochrome DNA (owner decision); A2-5 corpus = 3 docs (owner: ingest more);
constellations = copy-exact 20-catalog (backend 125 KB/32-body API mapping
rejected by user as visual regression — API integration deferred); visual
screenshot evidence for C1/C2/C7 parity grid still manual.

New authoritative reference (2026-09-29 13:05, 2,521 lines / 135 KB — replaces
`minimalist_monochrome_galaxy_database.html`, now demoted to secondary):

```
/home/xander/Documents/portfolio/mazzaroth_sovereign_database.html
```

**The 4 improvements** (user, verbatim intent — everything else = copy exactly):
1. **Galaxy**: ingest the entire database — all 1,105 `/api/galaxy` nodes as
   clickable stars (demo `MAJOR_STARS` is placeholder only), **+ slow auto-spin**
   (D10: reference look, but continuously rotating).
2. **Librarian**: connect to Ollama — through our backend `POST /api/ask` (SSE with
   grounded RAG + citations), not the reference's raw browser→`localhost:11434`
   call; keep the reference's chat UI, model button, status dot (fed by
   `/api/status`: `llm_online`, `available_models`).
3. **Maps**: real MapLibre (offline PMTiles via `/api/maps` + `/maps/{file}`),
   replacing the reference's drawn canvas — keep its HUD overlay styling.
4. **Capture all real data** across the program + **domain filtering between Bible
   and the other databases** (bible / medical / survival / literature / cognitive)
   in the galaxy + search.

Plus: **consistent dark/light parity** (both themes must render identical content —
the partial build failed this), 4 modules only (D11), offline (no CDN), brand
MAZZAROTH.

## A. Reference inventory & gaps

**In the reference (copy as-is):** 5 zones (yellow context-tools 72px / red workspace
/ green nav 32% "4 MODULES" / blue **Deep Reading Deck** 47% with font-scaling +
chapter paging / purple telemetry 21% + stack-depth); 4 view panels
(`view-galaxy`, `view-constellations`, `view-librarian`, `view-map`); complete
`:root` light + `[data-theme="dark"]` token sets (parity built-in); conditional
yellow tool groups per active view (galaxy: search/density/lines/recenter/zoom;
constellations: mode/season/position/lines/recenter; librarian: model select +
Ollama dot + clear-chat; map: graticule + center); constellation season/position
filters; librarian chat log + `librarian-input` + send; search modal ⌘K;
add-memory button; wireframe guide; toasts; `MAJOR_STARS` demo (chapters/tags/
connections) — behavior reference for the reading deck.

**NOT in the reference (build in C2–C6):** zero `/api/*` calls (demo data only);
no MapLibre (map is a drawn 2D canvas); Ollama called raw from the browser
(`fetch http://localhost:11434/api/generate` :1857 — bypasses our RAG/citations);
CDN Tailwind + Google Fonts (:8-12); no domain/Bible filter; no animation loop
(static 2D canvases); no Field Notes (per D11, fine).

**Salvage (keep):** backend + `data/*.db` (1,105 nodes / 1,157 links verified);
`web/vendor/` offline libs (tailwind.js, three, OrbitControls, maplibre-gl,
pmtiles); `reference/contract-probes.txt`. **Removed under de-bloat order
(2026-09-29):** `reference/old_frontend.js`, `_trash/`, unused leaflet/lucide/
tween vendor files, dead root `src/`, `tests/`, `galaxy/src`, `constellation/src`,
stale `target/release/{librarian,mazzaroth-gui}` binaries.

## B. Acceptance (definition of the user's "exactly like this")

1. Side-by-side with the reference: same zones, proportions, typography, labels —
   in **BOTH themes**, nothing missing in dark (user's complaint, hard gate).
2. 4 modules all functional with real data: galaxy=full DB+spin, constellations=
   real API stars+filters, librarian=Ollama-via-backend streaming, map=MapLibre
   offline tiles.
3. Domain filter (bible vs medical/survival/literature/cognitive) visibly filters
   galaxy stars and search results.
4. Deep Reading Deck loads real documents/scripture with provenance; font-scaling
   and chapter-paging buttons work.
5. Offline: devtools shows zero external requests; `cargo test --workspace` 7/7;
   plain `mazzaroth` launches + opens browser (C8).

## C. Decisions

D1–D9 (prior cycle) remain locked where still relevant (D3 chat=section body,
D8 search=nodes+BM25, D9 expand-on-read). **New, answered 2026-09-29:**
- **D10** — Galaxy motion: reference look **+ slow auto-spin** (continuous 2D
  canvas rotation, redraw loop; not Three.js).
- **D11** — Nav = **4 modules exactly as reference** (Field Notes UI deferred;
  backend endpoints stay untouched).
- **D4 (standing)** — Maps = MapLibre GL + PMTiles, monochrome-styled.
- **D6 (standing)** — No CDN: vendor Tailwind + fonts (fetch once) into
  `web/vendor/`; gate greps for external URLs.

## D. Backend contract (live-probed 2026-09-29 — JS must match exactly)

| Endpoint | Method / shape | Gotchas |
|---|---|---|
| `/api/galaxy` | GET → `{nodes, bodies, links, lines, total_nodes, timestamp}` | 1105/1157 live; `nodes`≈`bodies` |
| `/api/status` | GET → `llm_online, llm_endpoint, llm_model, available_models, profile, total_ram_mb, total_documents, total_chunks, total_notes, total_constellations, total_maps` | **no** `total_nodes/total_links/version` |
| `/api/search?q=` | GET → `[{chunk_id, doc_id, doc_title, …, snippet, rank}]` | **`doc_title`/`doc_id`** |
| `/api/ask` | **POST JSON `{question}`** → SSE `event:token` / `event:citations` (data = **array**) / `event:done` | GET=405; citations = raw array |
| `/api/memory/node` | POST `{label, tier?, content?, tags?, pos_*}` | no `category` key |
| `/api/notes` | GET/POST `{title, content}`; PUT/DELETE `/api/notes/{id}` | field `content` |
| `/api/read/{doc_id}`, `/api/documents`, `/api/categories` | GET | reader source |
| `/api/scripture` + `/languages` `/versions` `/meta` | GET | ids validated |
| `/api/maps` + `/maps/{filename}` | GET + Range | MapLibre PMTiles |
| `/api/sections/constellations` | GET | real constellation data (32) |
| static | `ServeDir("web/")` | `/vendor` `/css` `/js` |

Run: `./target/release/mazzaroth serve --bind 127.0.0.1:8080` from repo root.

## E. Stages (OWNS + gates) — copy-exact workflow

Prescribed layout: `web/index.html` (from reference, verbatim structure),
`web/css/app.css` (reference styles or kept inline — declare choice), split JS:
`web/js/api.js` (contract layer only), `web/js/galaxy.js` (galaxy view), 
`web/js/sections.js` (constellations/librarian/map), `web/js/app.js` (shell,
nav, tools, reader, overlays). Reference code may be copied verbatim into these
files; improvements applied on top.

**C0 — Freeze & salvage** [DONE — salvage ran its course; artifacts retired by
owner-ordered de-bloat 2026-09-29: `reference/old_frontend.js`, `_trash/` removed.
Surviving evidence: `docs/contract-probes.txt` (moved from `reference/`). No partial-C trash move
needed — it was rebuilt in place and verified under C1–C8.]

**C1 — Copy-exact shell** — OWNS: `web/index.html`, `web/css/app.css`.
- Copy the reference wholesale (structure, zones, views, modals, both theme
  token sets, conditional yellow tools, wireframe guide, toasts). Swap CDN
  `<script src="https://cdn.tailwindcss.com">` + Google Fonts `<link>` → local
  `/vendor/` equivalents (vendor the fonts once; declare in gate).
- Gate C1: `grep -q "MAZZAROTH // SOVEREIGN" web/index.html` &&
  `grep -q 'data-theme="dark"' web/index.html` &&
  `grep -rEo 'https?://[^"'\'' >]+' web/index.html | grep -v 127.0.0.1` EXPECT
  empty. EVIDENCE: screenshots of ALL 4 views × BOTH themes (8 shots) — parity.

**C2 — Galaxy: full database + spin + domain filter** — OWNS: `web/js/galaxy.js`.
- Replace `MAJOR_STARS` demo with `/api/galaxy` (all **1,105** nodes: hero stars
  = node positions/colors/categories, links → synapse lines; counts → purple).
- **D10**: continuous slow rotation of the stipple field (redraw loop,
  ≈1 revolution/3 min, pauses while dragging, resumes after).
- **Domain filter** (improvement 4): galaxy tool buttons
  bible/medical/survival/literature/cognitive (+ALL) filter displayed stars by
  `category`/`tier`; active domain highlighted; chip shows `SECTOR: <DOMAIN>`.
- Search pill → `/api/search` (BM25) + client node names (`doc_title` rows).
- Gate C2: console log `HERO_STARS=1105`; rotation verified in 10-s video;
  `grep -q "category" web/js/galaxy.js && grep -q "api/search\|search(" web/js/galaxy.js`.
  EVIDENCE: video + screenshots (ALL + one filtered domain).

**C3 — API contract layer** — OWNS: `web/js/api.js`.
- One module mirroring §D exactly: `getGalaxy, getStatus, search, askStream
  (POST {question} + array-citations SSE), createMemoryNode, getDocument,
  scripture*, maps*, constellations*`. All other files import from it — no raw
  `fetch` elsewhere.
- Gate C3: `grep -q "question:" web/js/api.js && grep -q "doc_title" web/js/api.js
  && grep -q "method: 'POST'" web/js/api.js` &&
  `! grep -rn "document_title\|note.body\|'body'," web/js`. EVIDENCE: field-diff
  table vs §D in `GATES.md`.

**C4 — Deep Reading Deck** — OWNS: `web/js/app.js` (reader part).
- Doc star / search hit → `/api/read/{doc}` into `reader-tag/id/title` + body
  (provenance footer: license/date/source); scripture drilldown
  language→version→book→chapter via `/api/scripture*`; wire `btn-font-dec/inc`
  and chapter paging; expand-on-read retained from reference layout.
- Gate C4: manual read of one manual chapter + one bible chapter.
  EVIDENCE: screenshots incl. provenance footer.

**C5 — Three data modules** — OWNS: `web/js/sections.js`,
`web/vendor/maplibre-gl.{js,css}`.
- **Librarian**: reference chat UI → `api.askStream` (SSE, streaming text,
  citation chips, refusal line); model button + status dot ←
  `/api/status.available_models`/`llm_online`; clear-chat kept.
- **Constellations**: reference canvas + season/position/lines filters → fed by
  `/api/sections/constellations` real data (not demo).
- **Map**: vendor MapLibre; replace drawn canvas with MapLibre GL map loading
  `/api/maps` PMTiles offline; keep reference's coordinate HUD + hint overlays;
  graticule/center tools adapt to MapLibre.
- Gate C5: `grep -q "api.askStream\|/api/ask" web/js/sections.js` &&
  `grep -q "maplibre" web/js/sections.js && ! grep -q "openstreetmap" web/js`
  && `grep -q "sections/constellations" web/js/sections.js`.
  EVIDENCE: streamed reply screenshot, offline map screenshot (network tab =
  localhost only), constellation view screenshot.

**C6 — Shell wiring & telemetry** — OWNS: `web/js/app.js`.
- Nav view switching (4 cards, spin pauses off-galaxy), yellow tool groups per
  view (from reference) wired to C2/C4 handlers, add-memory →
  `POST /api/memory/node`, ⌘K modal (nodes + BM25), purple telemetry from
  `/api/status` real fields (badge = `profile` + `llm_online`; rows =
  `total_documents/chunks/notes`; gauge = real ratio or omit), toasts on every
  action, theme toggle persists.
- Gate C6: `grep -q "api.getStatus" web/js/app.js && grep -q "memory/node"
  web/js/api.js`. EVIDENCE: action checklist run + both-theme screenshots.

**C7 — Regression + parity audit** (I run it) — live probes: §D table vs actual
JS calls; every §H1/§H2 failure mode re-checked (GET-ask, `body` field,
`document_title`, OSM, fake telemetry, XSS in snippet); 8-shot theme/view parity
grid; zero external requests.
- Gate C7: all §H items CLOSED with EVIDENCE in `GATES.md`; parity grid shot.

**C8 — Launcher** — OWNS: `install.sh`.
- Wrapper must `exec ./target/release/mazzaroth "$@"` (bin renamed in
  `crates/cli/Cargo.toml`) + open browser on fresh start
  (`(sleep 1; xdg-open http://127.0.0.1:8080 &) &` before exec).
- Gate C8: fresh shell: `mazzaroth` → server up + browser opens. EVIDENCE:
  transcript.

## F. Definition of done

C0–C8 gates green in `GATES.md`; §B.1–B.5 acceptance demonstrated with evidence;
`cargo test --workspace` 7/7; `mazzaroth` launcher works; zero external requests.

## G. Legacy milestones (M0–M7, all DONE)

M0 skeleton (L0) · M1 provenance/ingest (L1) · M2 FTS5+reader (L2) · M3 grounded
librarian (L3) · M4 galaxy graph (L4) · M5 offline PMTiles (L5) · M6 notes (L6) ·
M7 doctor+installer (L7). Backend of record.

## H. History — rejected prior frontends (regression source for C7)

- **R-series** (failed audit, 8 P0s): chat GET-vs-POST, citations array, notes
  `{body}`→422, reader never wired, OSM instead of PMTiles, empty constellations,
  stale launcher; fake telemetry, `document_title` mismatch, snippet XSS, missing
  fonts, stale synapse lines. Full detail:
  `/tmp/opencode/PLAN.pre-cleanroom.md` §H.
- **Partial C-build** (rejected 2026-09-29 by user): broken dark-mode parity,
  empty constellations, no MapLibre — superseded by copy-exact approach; archived
  to `_trash/c-partial/`.
