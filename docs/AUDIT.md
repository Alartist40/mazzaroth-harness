# AUDIT.md — Mazzaroth Full Audit (2026-09-29)

Read-only audit of the working tree. **No code was edited.** Supersedes the 2026-09-28
audit (its false claims are itemized in §7).

**Verification re-measured, not assumed:**

| Check | Result |
|---|---|
| `cargo check --workspace` | Compiles, **1 warning**: `crates/cli/src/main.rs` is target of both bins `librarian` and `mazzaroth` |
| `cargo test --workspace` | **7/7 integration tests pass** (not "8/8 test suites" — see §7) |
| Live server on 127.0.0.1:8099 | `/health` OK; all API endpoints probed; `data/librarian.db` = **3 docs, 9 chunks, 0 notes** |
| `data/mazzaroth.db` | **Orphaned old-schema DB: `nodes` = 1,097 rows, `links` = 1,151 rows — never read by any active code** |
| Traversal probe | `GET /api/scripture/versions?lang=../../../constellation/data` → **200 `["constellations"]`** (live-confirmed) |

---

## P0 — "Stars no longer appear for every individual data node" (root cause)

The old system stored **1,097 individual memory nodes**, one star each (`src/engine.rs:120-141`;
HUD placeholder `web/index.html:34` "VAULT: 1,097 STARS"). The `crates/` restructure silently
dropped both the data and the rendering path.

1. **Data never migrated.** Active binary reads `data/librarian.db` (documents schema,
   `crates/cli/src/main.rs:19`). The 1,097-node vault lives in `data/mazzaroth.db`
   (`nodes`/`links` tables) which **no active code can read** — `crates/core/src/db.rs` only
   knows `documents/chunks/notes`. Galaxy now renders **7 stars** (1 core + 3 categories +
   3 docs) instead of ~1,097. *This is the regression.*
2. **Edge contract broken.** `/api/galaxy` returns `nodes`+`links`
   (`crates/server/src/routes/galaxy.rs:34-38`), frontend renders edges from `data.lines`
   (`web/js/main.js:348`) and never reads `.links` → **zero galaxy edges drawn**
   (category→doc, note→doc backlinks). `crates/server/tests/galaxy_test.rs:54` asserts the
   backend's own shape, so tests stay green while the UI draws nothing.
3. **"+ Ingest Memory Node" is fake.** `formNewMemory()` (`web/js/main.js:1418-1442`) only
   pushes a client-side object — no POST. The 12 s poll (`web/js/main.js:113`) wipes the
   star on the next refresh.
4. **GPU leak every 12 s.** Each poll recreates a CanvasTexture and a 32k-particle
   BufferGeometry with no `.dispose()` (`web/js/main.js:348-349, 390-453, 258-320`) →
   memory grows until stars degrade/vanish, worst on SBC targets.
5. **Unbounded spiral radius.** Doc nodes at `r = 200 + idx*35` with no cap
   (`crates/server/src/routes/galaxy.rs:105`); camera `maxDistance = 6500`
   (`web/js/main.js:93`) + `FogExp2(0.00018)` (`web/js/main.js:77`) hide anything past
   ~8,000 units. At 3 docs harmless; at ~1,097 docs outer stars sit at r≈38,000 —
   invisible and unreachable even after migrating the data back.
6. *Latent:* any `NaN` coordinate makes `Points.boundingSphere` NaN and frustum culling
   drops the entire star cloud; no finite-number guard in `renderDatabaseStars`
   (`web/js/main.js:402-412`).

**Fix direction (awaiting approval):** migrate `nodes`/`links` into the galaxy graph (or
teach `handle_galaxy` to read the old schema), emit `lines` alongside `links`, cap/normalize
spiral radius into ~0..1200, POST real memory nodes server-side, dispose GPU resources
before re-creation.

---

## P1 — Layout: overlapping / messy UI

1. **Both ambient HUD overlays are dead or colliding.** `.ambient-hud-top-left`
   (`web/css/base.css:96-103`, z-index 5) starts y=60 and is covered by the sidebar
   (y=66, z-index 20, `base.css:265-281`) → clipped behind the panel.
   `.ambient-hud-bottom-left` (`base.css:115-126`, bottom:16/left:20) is covered by the
   sidebar in galaxy/constellations **and** by `.workspace-panel` (left:20, z-index 30,
   `base.css:556-571`) in librarian/maps/notes → **never visible in any section**, only
   bleeding through 0.92-alpha panels as blur-noise.
2. **Right HUD vs left sidebar collide on narrow screens.** Sidebar x=20..380, HUD
   x=(W−460)..(W−20) → overlap below ≈860 px. Only media query shrinks widths at 900 px
   (`base.css:1163-1170`) — no stacking, no mobile layout.
3. **Top bar overflows.** `space-between`, no wrap (`base.css:131-146`) with brand + 5 tabs
   + status pill + 2 buttons (`web/index.html:38-82`) ≈ 1,200 px content — clips/crushes
   below ~1,150 px with no responsive handling.
4. **HUD content clipped, no scroll.** `.hud-panel` is `overflow: hidden` (`base.css:936`)
   but `#hud-metadata-view` has no overflow rule — long desc + tags + waveform + grid can
   push the `CLOSE RECORD` button (`web/index.html:365`) out of view permanently on short
   viewports.
5. **CSS referenced but never defined:** `.scanline` (`web/index.html:299`),
   `.animate-orbit` / `.animate-orbit-reverse` (`web/index.html:316,320`) — **zero
   `@keyframes` in base.css** → HUD satellites frozen, scanline invisible.
6. **Dead/blank widgets:** `#waveform-canvas` is never drawn to (no reference in main.js,
   "432.8 Hz" is a static fake); `#selection-overlay-svg` is only ever cleared
   (`web/js/main.js:1406`) — click-reticle feedback from commit 3bf9b85 is gone; stale
   placeholders until first interaction: `ALPHA CENTAURI`, `0xF72D`, `341.6 AU`
   (`web/index.html:304,328-331`), `VAULT: 1,097 STARS` (`:34`), `LEVEL 1: CATEGORY`
   (`:125` — JS writes "STEP 1: …").
7. **Stale chrome state across section switches.** `switchSection('galaxy')` does not reset
   `drilldownState.step` (`web/js/main.js:184-193`; HEAD commit 80f02d6 did) → stale
   chapter list after switching away and back. `sidebar-count-badge` updates only in
   constellations mode (`web/js/main.js:199,918`) → galaxy view shows leftover
   "32 CELESTIAL" / "N MATCHING"; initial "32 SECTORS" (`web/index.html:89`) never changes.
8. **Reading text cannot be selected/copied.** `body { user-select: none }`
   (`base.css:38`) has no override for `.reading-content` / chat — a reader app whose
   scripture and manual passages cannot be highlighted.
9. **Consistency noise:** commit ff21e09 claims "remove emojis" yet every control is
   emoji-laden (`web/index.html:47-64,102-119,139-163`) while `cleanLabel` strips emoji
   from data labels — split visual language. `CLOSE RECORD` jumps position between
   metadata mode and reading mode (only `#reading-pane` is `flex:1`, `base.css:1083-1089`).
10. **Wasted render cycle:** 3D scene keeps rendering + auto-rotating behind full-screen
    workspaces (`animate()` unconditional, `web/js/main.js:1444-1456`) — CPU burn on the
    Pi-class targets this ships to.

---

## P1 — Functional bugs (frontend)

1. **Every AI answer is prefixed with raw JSON.** SSE parser handles `event: citations`
   but never skips its `data:` line; next iteration matches the token branch and appends
   raw citations JSON into the chat bubble (`web/js/main.js:1061-1091`). Server emits
   citations before the empty-hit check (`crates/server/src/routes/ask.rs:57-62`) →
   happens on **every** reply, including refusals.
2. **Bible drilldown dead-ends silently.** Galaxy API has no `:lang:` bodies → step 1
   always shows the hardcoded fallback (`web/js/main.js:526-531`) including **`grc`, which
   does not exist** (verified: `versions?lang=grc` → `[]`, meta → 404, chapter → 404) →
   fallback books → `openBibleChapter` does `if (!res.ok) return;` (`web/js/main.js:763`)
   → **clicking a chapter does nothing, no error**. Real languages work (`eng`, `spa`
   verified).
3. **Search box never searches.** `onSearchInput` only re-filters the sidebar list
   client-side (`web/js/main.js:987-997`); `/api/search` (BM25 flagship feature) is never
   called by the shipped UI. It also re-fetches `/api/documents` **on every keystroke**
   (`web/js/main.js:660`) with no race guard.
4. **Offline Maps section is inert and self-contradictory:** tiles come from live internet
   (`tile.openstreetmap.org`, `opentopomap.org`, `cartocdn.com` — `web/js/main.js:1115-1170`);
   no PMTiles client exists, `/maps/{filename}` never called; initial layer is OSM streets
   while UI shows "Topographic" checked (`web/index.html:253` vs `web/js/main.js:1115`);
   package "VIEW" button has **no click handler** (`web/js/main.js:1144-1155`); `maps/`
   dir doesn't exist → `/api/maps` always `[]`.
5. **`selectDomain('cognitive')` lists ALL documents** — filter is
   `d.category === domain || domain === 'cognitive'` (`web/js/main.js:663`). "Literature &
   Sci" card is always empty (no such category in DB).
6. **Notes failures are silent.** `saveCurrentNote` only handles `res.ok` — 4xx/5xx shows
   nothing (`web/js/main.js:1234-1241`); empty title returns silently (`:1223`);
   `deleteCurrentNote` swallows everything (`:1244-1253`).
7. **HTML injection from ingested data.** `cleanLabel` does not HTML-escape, yet its output
   goes into `innerHTML`: `d.retrieved_date`/`d.license` (`web/js/main.js:686`), bible
   `b.name` (`:616`), version stems (`:572`), constellation labels (`:933`, `:974`),
   language labels (`:541`). Backend validates only non-emptiness
   (`crates/core/src/content.rs:49`), so `retrieved_date: "<img src=x onerror=…>"` executes
   in the browser. Note titles/content *are* escaped — inconsistent.
8. **No Escape-key to close HUD, no click-empty-to-deselect** — zero `keydown` handlers in
   main.js; close only via ✕ or the possibly-clipped CLOSE RECORD button.
9. **Race:** the 12 s interval fetch can resolve after a section switch and render with the
   wrong dataset for one tick (`web/js/main.js:325-355` captures API at request time,
   section at response time).

---

## P2 — Backend bugs (code-verified)

1. **Path traversal is live.** `lang`/`version` joined unchecked into `galaxy/data/bibles`
   (`crates/core/src/scripture.rs:53-63,92-94,186-201`); confirmed
   `versions?lang=../../../constellation/data` → `["constellations"]`. `version=/abs/path`
   also escapes (`PathBuf::join` with absolute path replaces base). Fix: whitelist
   `^[A-Za-z0-9_-]+$` + canonicalize under base.
2. **Ingest panic on multibyte text.** `text[start..end]` byte arithmetic with no
   `is_char_boundary` check (`crates/core/src/content.rs:130,140`) panics on UTF-8
   boundaries > 3,200 bytes. Panics aren't `Err`, so `librarian ingest`
   (`crates/cli/src/main.rs:69`) and server auto-ingest (`:113`) crash.
3. **No timeout on the LLM call** — `reqwest::Client::new()` (`ask.rs:94`): wedged Ollama
   hangs the SSE task forever. Reachability probes (~1.6 s) run *before* headers
   (`ask.rs:51`, `state.rs:36-52`) → sluggish TTFB on every question.
4. **Status says "online" while answers silently degrade.** Probe accepts OpenAI-compatible
   `/v1/models` (`state.rs:44`) but generation only speaks Ollama `/api/generate`
   (`ask.rs:93-106`) → badge shows "Ollama Online", reply falls back to a canned string.
   README's "OpenAI-compatible endpoint" claim unsupported.
5. **`status.rs:49` hardcodes `read_dir("maps")`**, ignoring `config.maps_dir` → wrong
   `total_maps` whenever `--maps-dir` is used (its own test uses `--maps-dir`).
6. **PMTiles Range parsing** (`routes/maps.rs:65-72`): 0-byte file underflows
   `total_len - 1` (debug panic); suffix ranges `bytes=-500` silently become `0-499`;
   multi-range returns whole file as bogus 206; plus blocking full-file `read_to_end`
   inside `async fn` (`maps.rs:54,81-106`) buffers entire .pmtiles files on the runtime.
7. **Ollama NDJSON parsed per network chunk** (`ask.rs:106-124`) — tokens split across
   chunks are dropped; multi-line SSE `data:` loses newlines → words jam together.
8. **DB errors hidden as empty results:** `unwrap_or_default()` at `ask.rs:37`,
   `galaxy.rs:44,65`, `status.rs:44-46`, `notes.rs:108,127` — a DB failure looks like
   "0 documents" or "I don't have that in the library." `Mutex::lock().unwrap()` on every
   DB call (`db.rs:66,153,…`) → one poisoned lock 500s all later requests.
9. **Everything is CWD-relative** (`config.rs:78-84`, `state.rs:14`, `lib.rs:13-21`,
   `status.rs:49`, `main.rs:111-113`) — run the installed binary outside the repo and you
   silently get: no UI (fallback serves nonexistent `web/`), no scripture, 0 maps.
10. **`reindex` doesn't reindex** — only counts rows (`crates/cli/src/main.rs:75-79`), no
    FTS rebuild, no VACUUM. **`doctor` lacks the disk + FTS5-integrity checks** its help
    text and GATES L7 claim (`main.rs:147-196`).
11. **Hostile defaults for a sovereign/local tool:** bind `0.0.0.0:8080` +
    `CorsLayer::permissive()` (`crates/server/src/lib.rs:49`) + zero auth → any web page
    a user opens can read/create/delete notes and drive `/api/ask` over the LAN. Suggest
    default `127.0.0.1` + same-origin CORS.
12. **Dead/duplicate API surface:** `/api/memory/celestial` duplicates `/api/galaxy`
    (`lib.rs:30-31`); `/api/sections`, `/api/search`, `/api/categories`,
    `/api/scripture/languages`, `/maps/{filename}` never called by the shipped UI;
    `routes/mod.rs:31` advertises `/api/ask` in a GET listing though it is POST-only;
    backend BM25 search unreachable from UI (see P1-F3).
13. **All scripture errors map to 404** (`routes/scripture.rs:38,54,76`) — parse failures
    and internal I/O masked; `list_versions` never errs, so the 404 branch at `:76` is dead.
14. **Unused workspace deps:** `rust-embed`, `mime_guess` (`Cargo.toml:32-33`, zero
    references in any `.rs`).

---

## P2 — Ledger & test honesty gaps

- **GATES L7 EVIDENCE overclaims** ("RAM profile check, database stats, LLM probe
  verified"): `crates/cli/tests/doctor_test.rs:29-39` never calls `run_doctor` — it
  asserts row counts and profile parsing only.
- **L3 test never asserts SSE framing** the frontend parser depends on — that is exactly
  why the citations double-processing bug (P1-F1) shipped green.
- **L4 test codifies the shape the UI does not read** (`nodes`/`links`), masking P0-2.
- **L5 test covers only `bytes=0-6` happy path** — no 416 / suffix / empty-file /
  traversal cases (all of which are broken, P2-B1/B6).
- **Root `tests/*.rs` (5 files: cognitive, physics, MCP, corpus, store) never run** — root
  manifest is a virtual workspace; they `use mazzaroth::…`, a lib that no longer exists.

---

## P3 — Documentation claims that are false

| Claim | Reality |
|---|---|
| AUDIT.md (prev) "0 warnings" | Duplicate-bin warning on every cargo invocation |
| AUDIT.md (prev) "8/8 integration test suites" | 7 test targets exist; the 8th gate (L0) is `cargo check` |
| AUDIT.md (prev) layout: `ui/` is the frontend | Server serves `web/`; `ui/src` is only a fallback (`crates/server/src/lib.rs:13-21`) and is a divergent older copy (576 vs 1,456 lines) |
| AUDIT.md (prev) "Maps … active" | `maps/` directory does not exist → endpoint always `[]` |
| AGENTS.md describes `src/`, `tests/` as architecture | Nothing builds them; `cargo test` cannot see root `tests/` |
| AGENTS.md "32 classical asterisms **and** 12 Zodiac" | 32 total (12 zodiac + 20 asterisms) in `constellation/data/constellations.json` |
| README "Works completely offline" / "self-contained" | `web/index.html:9-15` loads three.js, OrbitControls, Tween, Leaflet from 3 CDNs — cold-cache offline = blank page; no 2D fallback exists despite README:14 |
| README "Opens http://127.0.0.1:8080 in your browser" | Server only logs the URL; no browser-open code anywhere |
| README CLI block | `--profile` shows no possible-values (clap lacks `ValueEnum`); `--dev-ui` omitted; usage line differs from real `--help` |
| README `reindex` / `doctor` descriptions | Count rows only; no disk check, no FTS integrity (see P2-B10) |
| README "OpenAI-compatible endpoint" | Probe only; generation is Ollama-only (P2-B4) |
| README "Markdown note-taking" / PLAN "markdown editor" | Plain `<textarea>`, no renderer; backlinks never clickable in UI |
| README API table | Missing 9 registered endpoints the shipped UI actually calls (`/api/status`, `/api/scripture*`, `/api/sections*`, notes by id, …) |
| docs/SCHEMA.md FTS5 | Documents external-content form (`content=`/`content_rowid=`); code creates standalone FTS5 (`crates/core/src/db.rs:100-105`) and omits triggers `chunks_ai/ad` (`db.rs:124-132`) |
| IMPLEMENT.md "UI (`ui/src/`)" | Shipped UI is `web/`; `ui/src` is the stale fallback |
| `tools/*.rs` usage lines | `cargo run --bin ingest-fm` etc. — no such bin targets exist; `tools/` has no manifest |

---

## P3 — Orphan / dead code inventory

| Path | Built? | Served? | Note |
|---|---|---|---|
| `src/` (11 files: engine, cognitive, server, store) | No | No | Old monolith; only HEAD's committed `Cargo.toml` built it |
| `galaxy/src/*.rs` | No | No | Reached only via dead `src/lib.rs`; **keep `galaxy/data/bibles/`** (runtime, git-ignored) |
| `constellation/src/*.rs` | No | No | Duplicated by `crates/core/src/constellation.rs`; **keep `constellation/data/constellations.json`** (`include_str!`) |
| root `tests/*.rs` (5) | No | No | Never executed by `cargo test` |
| `ui/` | n/a | No | Older divergent frontend; only kept alive as `dev_ui` fallback |
| `tools/*.rs` (3) | No | No | Usage commands reference nonexistent bins |
| `data/mazzaroth.db` (+wal/shm) | n/a | No | **The 1,097-node vault — the P0 data loss** |
| `target/release/mazzaroth-gui` | No | No | Leftover from removed GUI (no eframe/egui in lockfile — ban intact) |
| Unused JS globals | — | — | `lastRenderedCount`, `lastRenderedLinesCount`, `currentFocusNode` (set, never read), `librarianLabel` (`web/js/main.js:27-29,130`) |

---

## P3 — Git hygiene

1. **The entire new codebase is untracked — 44 files:** `crates/`, `content/`, `docs/`,
   `tools/`, `ui/`, `AGENTS.md`, `IMPLEMENT.md`.
2. **Committing the modified `Cargo.toml`/`Cargo.lock`/`README` *without* `git add crates/`
   produces a broken repo** — `Cargo.toml:1-6` declares members that don't exist in HEAD;
   fresh clone fails with "failed to load manifest for workspace member".
3. `Cargo.lock` modified (302 lines) — must land with `crates/` or `install.sh:10`
   (`--locked`) breaks.
4. Pending `web/` changes (3,503 changed lines) uncommitted while `ui/` twin is untracked —
   inconsistent frontend state in history.
5. `.gitignore` gaps: no `maps/` rule (first .pmtiles shows untracked), no root `*.db`,
   no editor/OS junk. (`data/*.db` and `galaxy/data` correctly ignored.)
6. No CI anywhere — all GATES verification is manual.

---

## Suggested fix order (when you approve implementation)

1. **P0-1/2** — migrate the 1,097-node vault + fix `links`→`lines` contract (restores stars & edges).
2. **P1-F1** — SSE citations skip (`i++` after citations data line) — one-line UX fix.
3. **P1-L1/L2/L3/L4** — layout pass: relocate/remove ambient HUDs, sidebar+HUD collapse rule, top-bar wrap, HUD scroll.
4. **P0-3/4/5** — memory-node POST, dispose() on rebuild, cap spiral radius.
5. **P2-B1** — scripture path whitelist (live traversal).
6. **P1-F2/F3/F4** — bible fallback dead-end, search wiring, maps reality-check (offline tiles or honest labeling).
7. **Ledgers/docs** — re-run and re-evidence GATES L0–L7; correct README/AGENTS/AUDIT/SCHEMA claims; decide fate of orphan dirs.
8. **Git** — one commit: `git add crates/ content/ docs/ tools/ ui/ AGENTS.md IMPLEMENT.md` + modified files together, or the repo ships broken.

**Open decision for the owner:** is `data/mazzaroth.db` (1,097 nodes) the intended vault?
Everything in P0-1 depends on that answer.

---

# AUDIT-2 — Hand-fix verification (2026-09-29, afternoon build)

Read-only verification of the user's hand-fixed frontend (files dated 09-29
14:11–14:51). **No code was edited.** All probes live against `127.0.0.1:8080`.
`cargo check` clean (exit 0). This section supersedes nothing above; it reports
what the hand-fix got right (§V) and what still blocks C-gates (§F).

Severity summary:

| # | Sev | Finding | Status |
|---|-----|---------|--------|
| A2-1 | P0 | MapLibre view is a fake canvas — no `maplibregl.Map`, no `.pmtiles` data, no pmtiles JS lib | blocks C5 |
| A2-2 | P0 | Reader never displays real ingested content (`doc.content` vs actual `structure[]`) | blocks C4 |
| A2-3 | P1 | Constellations canvas ignores backend (125 KB real data fetched, `console.log` only) | blocks C3 |
| A2-4 | P1 | Add-memory failure toast lies: "Saved locally" when POST failed | fix |
| A2-5 | P1 | RAG corpus = 3 documents / 9 chunks — "capture all the data" unmet for librarian | data action |
| A2-6 | P2 | Galaxy discards stored `x/y/z`; layout reshuffles each reload; fabricates `STAR-####` filler if API short/fails | fix |
| A2-7 | P2 | `window` mousemove/mouseup listeners re-added on every `initConstellations`/`initMap` | latent leak |
| A2-8 | P2 | Theme toggle leaks GPU (rebuilds without `dispose()`) | fix |
| A2-9 | P2 | Domain colors (sky/green/amber/purple) violate monochrome DNA (D10 copy-exact) | decision |
| A2-10 | P3 | Fonts declared (`Space Grotesk`/`JetBrains Mono`) but never loaded — system fallback ≠ reference | fix |
| A2-11 | P3 | Duplicate theme tokens (inline `<style>` + `app.css`); `--core-stroke`/`--core-disc-stroke` mismatch, zero consumers | cleanup |
| A2-12 | P3 | ⌘K search queries BM25 chunks only — star labels not searchable (D8 said nodes+BM25) | fix |
| A2-13 | P3 | `askStream` can fire `onDone` twice (explicit call + `finally`) | latent |

## A2-1 (P0) — MapLibre never instantiated (triple root cause, all three verified)

1. **Frontend never creates a map.** `web/js/sections.js:378` declares
   `let maplibreInstance = null;` — never assigned anywhere (grep: 0 writes).
   `initMap()` (`sections.js:685`) draws a 2D canvas with `rect()` placeholder
   continents. `index.html:545` has `<div id="maplibre-container">` (unused),
   `maplibre-gl.css`/`maplibre-gl.js` are vendored and loaded
   (`index.html:10,13`) but inert — zero `new maplibregl.Map` in `web/`.
   The 4 geocache pins are tagged `["GEOCACHE","MAPLIBRE","OFFLINE_VAULT"]`
   (`sections.js:810`) — a **false affordance**: no MapLibre is involved.
2. **No data.** `state.config.maps_dir` = `maps/` (`crates/core/src/config.rs:80`);
   directory does not exist; `GET /api/maps` → `[]`; zero `*.pmtiles` on disk.
3. **No protocol library.** Reading PMTiles over HTTP Range requires the
   `pmtiles` JS lib (`protocol.register('pmtiles', ...)`) — not in `web/vendor/`.

**Backend is ready** (`crates/server/src/routes/maps.rs`): list + serve routes,
filename traversal-safe, proper `Range`/`Content-Range`/`Accept-Ranges` handling.
`handle_serve_pmtiles` 404s correctly for missing files (probed).

**Fix path (3 steps, frontend-only + one data drop):**
1. Vendor `pmtiles.min.js` into `web/vendor/`, add `<script>` before `app.js`.
2. `mkdir -maps/`, drop ≥1 offline region `.pmtiles` (e.g. Natural Earth
   countries/states built with `planetiler`/`tippecanoe`) → verify
   `GET /api/maps` lists it and `Range: bytes=0-1023` returns 206.
3. In `initMap()`: fetch `/api/maps`, if non-empty build
   `new maplibregl.Map({container:'maplibre-container', style:{sources:{...pmtiles
   protocol...}}, center/zoom, attributionControl:false})` + the HUD overlay
   styling from the reference; if empty → keep canvas **but remove the
   `MAPLIBRE` tags** (honest labeling) and show `NO PMTILES IN maps/`.

## A2-2 (P0) — Reader discards real content; shows fabricated chapters

`app.js:272` guards on `doc.content` — **the field does not exist**.
`GET /api/read/{doc}` returns `ContentDocument {id,title,category,language,
provenance,structure:[ContentChapter{id,title,sections:[ContentSection{id,title,
text}]}]}` (`crates/core/src/content.rs:32-53`; live-probed on `kjv-genesis` —
real verse text present). Consequences:

- Condition `doc && doc.content` is **always false** → real corpus (FM 21-76,
  FM 4-25, KJV Genesis) is fetched and thrown away.
- Every star therefore renders the hardcoded fallback
  (`app.js:289-291`, `galaxy.js:216`): *"Galactic epicenter…"* /
  *"Chapter 2: Contextual link associations…"* — **fabricated text presented
  as the record** (hype pattern; the provenance footer says
  "INTEGRITY VALIDATED" over text that came from nowhere).

**Fix path:** map the real shape —
`currentNode.chapters = doc.structure.map(ch => ch.sections.map(s =>
s.title + '\n' + s.text).join('\n\n'))`, set `currentNode.provenance =
doc.provenance`, then `renderChapterBody()`. Wire `btn-chapter-prev/next`
across the full `structure[]` (buttons exist: `index.html:716,718`).

## A2-3 (P1) — Constellations ignore the backend

`sections.js:389-394`: `loadRemoteConstellations()` fetches
`/api/sections/constellations` (live: ~125 KB, 32 constellations, per-body
`constellation_id`, color `#ffcc00`) and only `console.log`s `data.length`.
The canvas renders the hardcoded 20-entry `CONSTELLATIONS_CATALOG`.
**Fix path:** on fetch, rebuild the catalog from the API payload (keep
monochrome styling — do **not** adopt the API's `#ffcc00`, monochrome is D10);
fall back to the hardcoded catalog if the fetch fails.

## A2-4 (P1) — False success toast

`app.js:523-524`: `catch` → `showToast("Saved locally: '<label>'")` after
`POST /api/memory/node` **failed** — nothing was saved. Fix: error toast.

## A2-5 (P1) — Librarian corpus is 3 documents / 9 chunks

`content/pd-demo/` holds exactly the 3 ingested JSONs; `documents` table = 3.
Galaxy side is complete (1,105 nodes ✓) but RAG citations/search beyond these
three docs return nothing. Action (owner, not code): ingest the remaining
corpus, then re-probe `/api/search`.

## A2-6 (P2) — Galaxy layout ignores stored coordinates + fabricates filler

- `galaxy.js:246-259`: every star gets a fresh `Math.random()` spiral
  position; DB `x/y/z` (real stored coords, ±638) never read → **layout is
  different on every reload** and disconnected from the stored graph.
- `galaxy.js:238-245`: if `/api/galaxy` returns <1,105 nodes the code
  invents `STAR-#### "Memory Star #N"` nodes with round-robin domains; on
  total API failure it renders **1,105 fabricated stars silently**. Currently
  masked (API returns exactly 1,105), but it is a dishonest fallback.
- Fix: prefer `node.x/y/z` when present (deterministic layout), spiral only
  for missing coords; render exactly what the API returns, empty-state at 0.

## A2-7/A2-8 (P2) — Leaks

- `sections.js:652,660` and `:826,842`: `window.addEventListener` inside
  `setup*Interactions` — re-added on every `initConstellations`/`initMap`
  call (currently once at boot; becomes a real leak the moment init re-runs).
  Fix: `AbortController` or a `bound` guard flag.
- Galaxy rebuilds (theme toggle, density, selection passes) never call
  `geometry.dispose()`/`material.dispose()`/texture dispose → GPU memory
  grows per toggle. (Prior audit P0-3 — still open.)

## A2-9 (P2) — Monochrome violation (needs owner call)

`galaxy.js` `getDomainColorObj`: bible=`#38bdf8`, medical=`#22c55e`,
survival=`#fbbf24`, literature=`#c084fc`. Reference (D10 copy-exact) is
strictly ink/paper monochrome; domain separation in the reference is done by
**filter UI**, not star color. Decision: (a) revert to monochrome (copy-exact),
or (b) keep colors as an explicit, documented deviation from D10.

## A2-10..A2-13 (P3)

- **Fonts:** `Space Grotesk`/`JetBrains Mono` declared in 3 places, no
  `@font-face`, no font files, no Google Fonts link (the reference used the
  CDN) → silent system-font fallback; typography differs from reference.
  Offline-correct fix = vendor the two woff2 files.
- **Token duplication:** identical `:root`/`[data-theme]` blocks live in
  `index.html:14-45` (inline, wins) and `app.css:1-39`. Dark defines
  `--core-stroke`, light defines `--core-disc-stroke`; neither is consumed
  anywhere (`var(--core` = 0 matches). Single-source in `app.css`, drop both
  dead names.
- **⌘K search** returns BM25 chunk hits only (`app.js` search rows) — a star
  label with no indexed chunk is unfindable (D8 said nodes+BM25; the galaxy
  pill search does cover nodes).
- **`api.js askStream`:** `onDone` can fire twice (explicit completion call +
  `finally` after reader end).

## §V — Verified GOOD in the hand-fix (evidence, not assumption)

- **DOM copy-exact achieved:** ID inventory diff vs reference = **0
  reference-only IDs**; current adds exactly 2 (`galaxy-tooltip`,
  `maplibre-container`). 826 lines vs 2,521 (reference is single-file; rest is
  split into `app.css` + 4 JS modules).
- **XSS-safe:** every `innerHTML` sink in `web/js/*` passes `escapeHtml()`
  or uses `innerText` (search rows, citations, tooltip, chat, provenance).
- **API contract correct:** `api.js` matches live probes (POST `/api/search`
  + `doc_title`, POST `/api/ask` SSE, `/api/status` real fields;
  `initTelemetry` reads `total_documents`/`total_chunks` — the fields that
  actually exist).
- **Shell complete:** all 4 tool groups (`tools-{galaxy,constellations,
  librarian,map}`), wireframe guide + 5 zone tags, toast, nav deck,
  3 prompt chips, Deep Reading Deck (`zone-blue`) with font ±/chapter
  prev-next/target-lock/copy-payload buttons wired, badges, spec counters
  (`spec-star-count`/`spec-link-count` filled at `app.js:558-559`), offline
  clock present.
- **Dark/light parity fixed:** symmetric token sets; theme toggle re-renders
  galaxy + constellations + map.
- **Backend:** `cargo check` clean; all probes 200; `/api/maps` Range logic
  present and correct; scripture ids still validated (traversal from prior
  audit not re-tested this pass — unchanged code).
- **Improvement over reference where it deviates:** reference `addMemFunc`
  wrote a *local fake star only* (no persistence) and called Ollama raw from
  the browser; current build persists via `POST /api/memory/node` and streams
  grounded RAG through `POST /api/ask` — keep both deviations (A2-4's toast is
  the only defect).

## Fix order (when authorized)

1. **A2-2** reader content mapping (pure frontend, unlocks C4 + stops
   fabricated text).
2. **A2-1** MapLibre 3-step path (vendor lib → `maps/*.pmtiles` →
   `new maplibregl.Map`; honest fallback label meanwhile) (unlocks C5).
3. **A2-3** constellations from API (unlocks C3) + **A2-4** toast truth.
4. **A2-6/A2-7/A2-8** layout determinism, listener/GPU leaks.
5. **A2-9** owner decision (monochrome vs documented color deviation).
6. **A2-10..13** fonts, token cleanup, ⌘K node search, `onDone` guard.
7. **A2-5** owner action: ingest remaining corpus; re-probe `/api/search`.

---

## AUDIT-2 RESOLUTION (2026-09-29 evening — implementation authorized)

User granted code-edit permission ("work on the code… download maplibre and add
it to the map section… remove any old code/ bin"). Findings status:

| ID | Status | Evidence |
|----|--------|----------|
| A2-1 MapLibre P0 | **FIXED** | `bootMapLibre()` in `sections.js` creates `new maplibregl.Map` with vendored `pmtiles.Protocol` (`web/vendor/pmtiles.js`, correct callback API for maplibre-gl 3.6.2); `maps/world.pmtiles` generated offline (Natural Earth 110m countries, z0–z6, 399 KB, 2,939 tiles, GZIP MVT); `#maplibre-container` added to `index.html`; geocache markers (7 pins), HUD coords, theme paint (`applyMapLibreTheme`), graticule GeoJSON layer, canvas fallback preserved. Verified: `/api/maps` → 1 entry; `Range bytes=0-1023` → 206; Node harness drove `pmtiles.Protocol` against live server: TileJSON (0–6) ✓, z0/z2/z6 land tiles decompress to MVT (`0x1a`) ✓, missing ocean tiles → empty MVT ✓ (protocol's documented MVT behavior); `grep "new maplibregl.Map"` ✓; C5 CHECK re-run → PASS. |
| A2-1 follow-up (2026-09-29, browser-verified) | **VERIFIED in real Chromium** | Two defects found & fixed via Playwright headless: (1) static files had no `Cache-Control` → browser heuristic cache served pre-MapLibre HTML/JS (user: "I see no map libre"); fixed with `SetResponseHeaderLayer` `cache-control: no-cache` on the router (`crates/server/src/lib.rs`), server rebuilt + restarted. (2) `maplibreInstance` never declared → strict-mode module ReferenceError killed boot after Map construction (chip showed "maplibreInstance is not defined — canvas fallback"); fixed with `let maplibreInstance = null` (`sections.js:826`). Added `#maplibre-status` chip instrumentation to `bootMapLibre()` (every failure branch reports itself). Post-fix headless Chromium run: chip `MAPLIBRE: world.pmtiles ACTIVE (z0–z6 offline)`, container visible, canvas hidden, 7 geocache markers, 0 page errors; dark + light screenshots both render wireframe world + graticule + pins + HUD (`/tmp/opencode/pwtest/map-view.png`, `map-light2.png`, `header-zoom.png`). |
| A2-2 Reader P0 | **FIXED by team** (prior turn) | `loadNodeIntoReader` maps `doc.structure[].sections[].text` → chapters; provenance footer from `doc.provenance`; C4 CHECK → PASS. |
| A2-4 False toast | **FIXED by team** | catch → `Memory save failed: ${e.message}`. |
| A2-13 Double onDone | **FIXED by team** | `askStream` `doneCalled`/`safeDone` guard in `api.js`. |
| A2-7 Listener leaks | **FIXED by team** | `constEventsBound` / `mapEventsBound` one-shot guards. |
| A2-3 Constellations API | **DEFERRED (user decision)** | copy-exact 20-catalog restored after user reported API-mapped look "messed up"; backend 125 KB/32-body fetch integration remains open if ever wanted. |
| A2-5 Corpus = 3 docs | OPEN (owner: ingest) | content/pd-demo only. |
| A2-6 Galaxy coords/filler | OPEN | random spiral + `STAR-####` synthetic filler unchanged. |
| A2-9 Domain colors | OPEN (owner decision) | monochrome DNA vs colored domains. |
| A2-10 Fonts never load | OPEN | no @font-face/font files/Google Fonts link. |
| A2-11 Token duplication | OPEN | inline `<style>` + `app.css` duplicate; `--core-*` dead. |
| A2-12 ⌘K node search | OPEN | BM25 only. |

**De-bloat executed (owner order, this session):** removed dead root `src/`,
`tests/`, `galaxy/src`, `constellation/src` (NOT `galaxy/data` — runtime scripture
dep at `state.rs:14`; NOT `constellation/data` — compile-time `include_str!` at
`constellation.rs:31`), `_trash/`, `reference/old_frontend.js`, unused
`web/vendor/{leaflet.css,leaflet.js,lucide.min.js,tween.umd.js}`, stale binaries
`target/release/{librarian,librarian.d,mazzaroth-gui,mazzaroth-gui.d,libmazzaroth.d,libmazzaroth.rlib}`.
Post-removal: `cargo check` exit 0; C1/C4/C5/C6/C8 CHECKs PASS; `GATES.md` C0
CHECK updated to surviving evidence (`reference/contract-probes.txt`).

**Verification commands:** `node --check web/js/{sections,app,api,galaxy}.js` all
pass; assets served 200 (`maplibre-gl.js`, `pmtiles.js`, `maplibre-gl.css`,
`css/app.css` geocache styles); served `sections.js` contains `bootMapLibre`.
Remaining manual evidence: browser screenshot of Map tab (both themes) + C1/C2/C7
8-shot parity grid.

---

## AUDIT-3 RESOLUTION (2026-09-30 — Full Feature & Unification Pass)

All major architectural, data-layer, cartographic, and UX requests fully shipped and verified:

| Milestone / Feature | Status | Implementation Details |
|---|---|---|
| **Street-Level Map Packs (M5)** | **SHIPPED & VERIFIED** | `mazzaroth maps fetch <bbox>` extracts bbox packages via official `pmtiles` binary directly from Protomaps planet build. Schema-aware styling detects vector layers for both overview and street-level packs. |
| **Map Downloader & CJK Glyphs (M5b)** | **SHIPPED & VERIFIED** | In-app 62-country dropdown downloader via background job (`POST /api/maps/fetch`), zoom buttons, chip deletion. 256-range Klokantech Noto Sans CJK glyph PBFs installed with strict sub-service routing (clean 404 on missing glyph, fixing `Unimplemented type: 4`). |
| **Offline Astrometry Engine (M8)** | **SHIPPED & VERIFIED** | 100% offline closed-form astronomical solver (`crates/core/src/sky.rs`) with Julian Day, GMST, LST, and Alt/Az coordinates for 115+ stars, 40+ constellation lines, and 800+ background stars. |
| **Knowledge Hierarchy Explorer** | **SHIPPED & VERIFIED** | Deep Reader overhauled with hierarchical drilldown (`Theme > Language > Book > Chapter > Passage`) and breadcrumb navigation (`#reader-breadcrumbs`, `#NEXUS-0` root). `GET /api/tree` endpoint registered. |
| **Star Navigation Handbook** | **SHIPPED & INGESTED** | 4 comprehensive chapters authored and ingested into `data/mazzaroth.db` (`total_docs=4, total_chunks=18`). |
| **Constellations & Dome Unification** | **SHIPPED & VERIFIED** | Unified Sky Deck and Constellations into `02 CONSTELLATIONS` across 4 primary system modules (`01 GALAXY`, `02 CONSTELLATIONS`, `03 LIBRARIAN`, `04 MAP`). Added workspace HUD dropdown for Live Dome, 3D Sphere, and Poster Grid modes. Unified tool rail with time simulation and coordinate filters. |

**Current Verification Status:**
- `cargo check --workspace` $\to$ Clean (0 errors, 0 warnings).
- `cargo test --workspace` $\to$ 8/8 test suites passing (`doctor_test`, `ingest_test`, `notes_test`, `galaxy_test`, `librarian_test`, `maps_test`, `search_reader_test`, `sky_deck_test`).
- `node --check` $\to$ Clean on all frontend JavaScript modules.

---

# AUDIT-4 — Full Re-Audit (2026-10-05, session "mazzaroth")

Read-only audit of the working tree. **No code was edited** (this file is the only artifact).
**60 findings: 3 P0, 8 P1, 26 P2, 23 P3.**

**Verification re-measured, not assumed:**

| Check | Result |
|---|---|
| `cargo test --workspace` | **14 passed / 0 failed** across 10 targets (8 integration suites + 6 core unit tests) — re-run 2026-10-05, not copied |
| Critical/high findings | **All 11 re-read at `file:line`** by the auditor (not trusted from subagent reports) |
| P0-3 date bug | **Independently reproduced** by re-running the exact `current_utc_ymd_h()` algorithm over 2026-01-01→2030-12-31 |
| Live server probing | **Not performed** — all findings are static-analysis observations (§8 labeling), except P0-3 |

## P0 — Critical

**A4-P0-1 — Constellations toolbar double-bound (4 buttons lie or no-op).**
`web/js/app.js:1641/1652/1661/1670` and `web/js/sections.js:651/657/658/660` both attach `click` to
`btn-const-mode`, `btn-season-cycle`, `btn-position-cycle`, `tool-const-lines`. Both bind paths run at
init (`app.js:99` → `setupToolbars()`; `app.js:109` → `initConstellations()` → `sections.js:464` →
`setupConstControls()`), verified. Consequences: lines toggle executes twice → state returns to
original (never changes) while the toast claims it did; LAYOUT button double-cycles so **SPHERE is
unreachable from it** and the toast always reports a different mode than actual; SEASON/COORD advance
two steps per click, desyncing button label, HUD badge, and real filter.
*Fix: one owner per control — delete the `app.js` copies (sections.js is the module).*

**A4-P0-2 — "Back to themes / HOME" navigation dead after first theme selection.**
`web/js/app.js:445` — `showHierarchyRoot()` computes `level: hierarchyState.selectedCategory ?
hierarchyState.level : 'root'`, and `selectedCategory` is never reset to null (assignments only at
`app.js:318,328,343,359,812,1872,1877`; verified by grep). `renderHierarchyExplorer()` branches on
`level` at `app.js:704/822/1015/1241/1458`, so every root entry point — `btn-tree-back-root`
(`app.js:894,987`), reader index button (`app.js:1589`), `tool-hierarchy` (`app.js:1698`) —
re-renders the current sub-level under a "Knowledge Themes & Hierarchy" header. Theme-root list is
unreachable without a full page reload.
*Fix: `showHierarchyRoot()` should reset `selectedCategory/selectedLanguage/selectedDoc/...` to null.*

**A4-P0-3 — `/api/sky` date math broken Feb 26–28 every year: debug panic / silently wrong sky.**
`crates/server/src/routes/sky.rs:71` deviates from Hinnant's algorithm — `doe/1020` and `doe/1461`
instead of `doe/1460` and `doe/36524`. Line 73 then computes `doy` in `u32` → subtract-with-overflow.
**Reproduced independently (exact algorithm, all dates 2026–2030): 17 broken days**, window drifts with
era position — `2026-02-26..28`, `2027-02-26..28`, `2028-02-27..29`, `2029-02-25..27`, `2030-02-28`.
Reachable with no input: bare `GET /api/sky` or any unparsable `time` falls through to
`current_utc_ymd_h()` (`routes/sky.rs:108,119`). Debug build: panic → 500 on the core endpoint.
Release build: wraps to garbage `m/d`, clamped to 12/31 at `routes/sky.rs:126-127` → **projects Dec 31
instead of Feb 26 with HTTP 200** (nav-critical silent wrong output). The UI masks it by always
sending `time` (`web/js/sections.js:611-614`); direct API/default calls do not.
*Fix: line 71 → `(doe - doe/1460 + doe/36524 - doe/146096) / 365`; add a unit test over the window.*

## P1 — High

**Backend**
- **A4-P1-1 — Whole-file RAM buffering + blocking I/O in async handler.**
  `crates/server/src/routes/maps.rs:106-108` (`read_to_end` for no-Range) and `:84-88`
  (`vec![0u8; length]` + `read_exact`) run synchronous `std::fs` on a tokio worker. `GET /maps/japan.pmtiles`
  (1.59 GB) = 1.59 GB heap per request; 2–3 concurrent unauthenticated requests ⇒ OOM + stalled runtime.
  *Fix: `tokio::fs` + streaming body (`ReaderStream`), keep Range branch streaming too.*
- **A4-P1-2 — HTTP Range parser violates RFC 7233 + zero-length panic.**
  `crates/server/src/routes/maps.rs:66-74`: `bytes=-500` (suffix range) parses `start=""` → serves the
  *first* 501 bytes as 206; `bytes=10-20,30-40` → `end` parse fails → whole tail returned; malformed
  values silently coerced instead of ignored/416. `:69/71/74` evaluate `total_len - 1` before the
  bounds check → `0 - 1` underflow panics (debug) against any 0-byte file in `maps/`.

**Frontend**
- **A4-P1-3 — Stale async responses clobber newer user selections.**
  `web/js/app.js:420-439`: `getDocument(docLookupId).then(...)` writes into the live `currentNode`
  with no request token/identity check — click star A then B, A's late response overwrites B's title,
  tag, chapters, and resets `currentChapterIndex`. Same unguarded pattern: `showHierarchyRoot()`
  (`app.js:467-475`) and `getScriptureMeta` (`app.js:1251-1253`) can replace an open chapter with the
  explorer view (or vice versa).
- **A4-P1-4 — Full celestial-dome repaint on every mousemove pixel.**
  `web/js/sections.js:1207` — `handleConstHover()` ends in unconditional `renderConstellations()`,
  invoked from window `mousemove` (`:1290-1296`) whenever Constellations is visible: 800+ background
  stars, each with a `createRadialGradient` (`:881`), plus labels/lines, re-painted per pointer move.
  *Fix: early-return when `closest` unchanged; throttle via rAF.*
- **A4-P1-5 — Galaxy render loop never pauses or cancels.**
  `web/js/galaxy.js:573` assigns `animFrameId`; `cancelAnimationFrame` appears nowhere (grep-verified).
  Hidden galaxy keeps rendering the 16k-particle scene + `controls.update()` at 60 fps in every other
  module, competing with MapLibre/2D canvas.
- **A4-P1-6 — No responsive layout: app is unusable below ~640 px.**
  `web/css/app.css` contains **zero `@media` queries** (grep `0`). At 375 px: `zone-yellow` (`w-16`, 64 px)
  + `right-column` (`w-80`, 320 px) + `main` padding exceed the viewport → `#zone-red` (all canvases and
  HUDs) collapses to ~0 px despite `<meta viewport>` being present.
- **A4-P1-7 — Telemetry contradicts its own labels and the docs.**
  `web/js/app.js:1955-1956` writes `N DOCS` into `#spec-star-count` (label **ACTIVE STARS**) and
  `N CHUNKS` into `#spec-link-count` (label **SYNAPSE LINKS**); `web/index.html:849` claims "4,500
  cataloged memory stars", `:918/:922` default to "1,400+"/"1,450+", and the engine renders 1,105
  (`web/js/galaxy.js:5`). Three mutually inconsistent star counts visible simultaneously.
- **A4-P1-8 — Reader hierarchy late-render race.** (folded into A4-P1-3; kept as separate trigger:
  `EXPLORE HIERARCHY` cold-cache click followed by an immediate chapter click swaps content twice.)

## P2 — Medium

**Backend (9)**
- **A4-P2-1 SSRF** — `routes/maps.rs:140` → `fetch.rs:373-381`: unauthenticated `source` URL from the
  request body passed verbatim as `pmtiles extract <source>` argument; no scheme/host allowlist.
- **A4-P2-2 No bbox area/size limit** — `fetch.rs:38-53` range-checks coordinates only; `[-180,-90,180,90]`
  + `maxzoom:15` accepted → unbounded download, disk exhaustion, one job slot pinned for hours.
- **A4-P2-3 Fetch job can hang forever** — no reqwest timeout (`fetch.rs:104,154`), no job timeout/cancel,
  `child.wait()` unbounded (`fetch.rs:444`). Stuck `running` state 409s **both** `POST /api/maps/fetch`
  and `DELETE /api/maps/{file}` (`routes/maps.rs:167-169`) until process restart.
- **A4-P2-4 Range parsing defects** — see A4-P1-2 (grouped; same root cause).
- **A4-P2-5 Zero-length file Range underflow** — see A4-P1-2 (grouped; same root cause).
- **A4-P2-6 No auth on state-changing endpoints + contradictory bind defaults** — `lib.rs:33-56`
  (`POST /api/memory/node`, `POST/PUT/DELETE /api/notes`, `POST /api/maps/fetch`, `DELETE /api/maps/{file}`)
  all unauthenticated; `config.rs:87` defaults `0.0.0.0:8080` while CLI defaults `127.0.0.1:8080`
  (`cli/src/main.rs:16`). High impact whenever `0.0.0.0` is used.
- **A4-P2-7 SSE drops bytes split across TCP chunks** — `routes/ask.rs:239`: `if let Ok(text) =
  std::str::from_utf8(&bytes)` discards the entire chunk on a multi-byte boundary split (CJK affected
  first-class); can swallow `done:true` → 45 s idle hang + `[stream interrupted after 45s idle]` on an
  otherwise complete answer. *Fix: `BytesMut` accumulation with `from_utf8_error().valid_up_to()`.*
- **A4-P2-8 `/api/search` limit unclamped** — `routes/search.rs:10,32` pass `usize` straight to SQL
  `LIMIT ?`; `limit > i64::MAX` fails `ToSql` → user-triggerable 500; huge limits materialize the whole
  corpus (full `text` + `snippet` per hit) while holding the global DB mutex.
- **A4-P2-9 Blocking I/O inside async, no timeout layer** — single `Mutex<Connection>` (`db.rs` ×15),
  no `spawn_blocking`, no `TimeoutLayer` in `lib.rs:15-86`; `/api/sky` re-parses the star catalog twice
  per request (`core/src/sky.rs:139,199,234`), `/api/status` does 800 ms ×2 + 1.5 s probes in-request
  and re-parses `constellations.json` (`routes/status.rs:25-52`).

**Frontend (17)**
- **A4-P2-10 GPU memory leak** — `web/js/galaxy.js:132-188,326-365,118-130`: geometries/materials/
  `CanvasTexture` rebuilt on star click (`:471-483`) and theme toggle without `dispose()` → monotonic
  GPU growth → eventual "WebGL context lost".
- **A4-P2-11 Fabricated stars on API failure** — `web/js/galaxy.js:239-245`: `Math.max(rawNodes.length,
  1105)` pads with fake `STAR-####` placeholders; total failure renders 1,105 invented stars with only
  `console.warn`. (Previously flagged A2-6, still present.)
- **A4-P2-12 No fetch timeouts anywhere** — `web/js/api.js` has zero `AbortController`/`signal` use;
  a stalled daemon leaves "Loading…" in the reader forever, download stuck at "● REQUESTING", ⌘K never
  resolving — no UI affordance that the request is dead.
- **A4-P2-13 Sky-projection race** — `web/js/sections.js:608-630`: `refreshSkyData()` assigns responses
  with no sequence guard; rapid `-1H` clicks resolve out of order → dome shows an arbitrary timestamp
  that contradicts the datetime field.
- **A4-P2-14 ⌘K search race, no debounce/abort** — `web/js/app.js:1781-1813`: per-keystroke requests;
  slower older response appends rows after newer ones → mixed results for two queries.
- **A4-P2-15 Unthrottled resize → repaint storms** — `web/js/app.js:118` + `:1960-1964`: every
  drag-frame fires full `renderConstellations()` (800+ gradients) + `maplibreInstance.resize()`.
- **A4-P2-16 Clipboard false success** — `web/js/app.js:266-273`: `navigator.clipboard.writeText`
  uncaught; toast "Reader text copied" fires synchronously even when permission is denied.
- **A4-P2-17 `node.id.startsWith` throw path** — `web/js/app.js:316`: a BM25 hit without `doc_id`
  (`api.js:21`) throws inside the row-click handler before `closeSearch()` (`:1892`) → modal stuck open,
  reader never updates.
- **A4-P2-18 Drag always ends in star selection (Galaxy)** — `web/js/galaxy.js:471-483`: `click` handler
  ignores `isDragging`, `maxRadius = 140` px → orbiting the camera selects the nearest star and silently
  replaces the reader.
- **A4-P2-19 Dome panning broken over stars** — `web/js/sections.js:1228-1249`: `mousedown` early-returns
  whenever `hoveredSkyStar` is set (no drag threshold) → drag over a star opens a folio instead of panning.
- **A4-P2-20 Blank map on failed remount** — `web/js/sections.js:1907-1918, 2140-2151`: `catch` claims
  "canvas fallback active" but never re-shows `#map-canvas` nor hides `#maplibre-container` → empty panel.
- **A4-P2-21 Global `user-select: none`** — `web/index.html:58` (`*`): reader/chat text cannot be
  highlighted or copied in a reading app; no `select-text` override anywhere.
- **A4-P2-22 Telemetry vs labels** — see A4-P1-7 (same root cause).
- **A4-P2-23 Dead backend contract** — `web/js/api.js:142 getConstellations()` never imported;
  `sections.js:8-352` renders a hardcoded 20-entry catalog while docs promise `/api/sections/constellations`
  (32 constellations) is consumed by the frontend.
- **A4-P2-24 Contrast failure** — `web/index.html:26/46` `--text-muted` ≈ 3.0:1 (light) / 3.3:1 (dark) on
  `text-[7.5px]`–`text-[9px]` labels → fails WCAG AA (4.5:1) on sub-10 px secondary text.
- **A4-P2-25 Primary navigation is mouse-only** — `web/index.html:728,747,763,779`: `.nav-deck-item` are
  plain `<div>`s (click handler only, `app.js:172-208`): no `tabindex`, `role`, Enter/Space handling →
  keyboard/screen-reader users cannot switch modules at all.
- **A4-P2-26 TARGET LOCK false success** — `web/js/app.js:296-303`: `selectGalaxyStar()` silently no-ops
  for non-galaxy ids (`galaxy.js:455-463`), toast fires unconditionally.

## P3 — Low

**Backend (13)**
- **A4-P3-1** `sanitize_region_name` keeps `..` (`fetch.rs:69-82`) → `name: "a..b"` creates a file that
  both serve (`maps.rs:47`) and delete (`maps.rs:155`) reject: orphan, un-deletable via API.
- **A4-P3-2** Fetch can overwrite protected `world.pmtiles` (`fetch.rs:384-387`; protection exists only
  in delete).
- **A4-P3-3** `/api/status` counts `read_dir("maps")` hardcoded, ignores `--maps-dir` (`routes/status.rs:52`).
- **A4-P3-4** `calculate_julian_day` `year - 1` overflow panic from `?year=-2147483648` (`core/src/sky.rs:74`;
  year unvalidated in `routes/sky.rs:109-117`).
- **A4-P3-5** `?time=2026-01-01Tnan` → `f64::clamp` passes NaN → HTTP 200 full of `null` coordinates
  (`routes/sky.rs:39-50,128`).
- **A4-P3-6** `.expect()` reachable in handler (`core/src/sky.rs:141`): malformed bundled JSON panics
  `/api/sky` on every request; star catalog parsed twice per request.
- **A4-P3-7** `?chapter=0` returns chapter 1 with 200 instead of 404 (`core/src/scripture.rs:393`).
- **A4-P3-8** `update_note` UPDATE + SELECT without transaction (`core/src/notes.rs:73-90`): concurrent
  DELETE turns a successful update into HTTP 500.
- **A4-P3-9** `Mutex::lock().unwrap()` ×15 in `db.rs` (+ `notes.rs`, `scripture.rs`): one panic while
  holding the DB lock poisons all DB endpoints for the process lifetime.
- **A4-P3-10** `/api/ask` can hang before first byte: `connect_timeout` only (`ask.rs:152-155,230`);
  the 45 s timeout starts only after response headers.
- **A4-P3-11** `extract_blocking` error path leaks pump threads + `pmtiles` child (`fetch.rs:437-446`):
  detached threads keep writing into a *later* job's log; child keeps writing `.tmp` reused by the next job.
- **A4-P3-12** CLI `maps fetch` bypasses the global fetch slot (`cli/src/main.rs:298-306`) → CLI + HTTP
  fetch of the same region write the same `.tmp` concurrently → corrupted archive; HTTP status shows nothing.
- **A4-P3-13** Minor info exposure/latency: raw `io::Error` to client (`routes/maps.rs:174`), fetch log
  with local paths/URLs, `/api/status` 1–3 s in-request probes + full-table loads for counters.

**Frontend (10)**
- **A4-P3-14** Hardcoded `http://127.0.0.1:8080` displayed as host (`web/index.html:610`) — wrong on any
  other host/port/HTTPS (display only; API calls are relative).
- **A4-P3-15** Promised-but-unwired controls: "ENTER TO JUMP" in search footer (`index.html:969`) — Enter
  in `#modal-input` does nothing; galaxy zoom `title="Zoom In (+)"` (`:267,279`) — no `+`/`-`/`=`/`_` key
  handler exists.
- **A4-P3-16** Sub-minimum touch targets: `#map-download-close` 20×20 (`index.html:676`), tool buttons
  36×36 vs 44 px guideline, `text-[7.5px]` labels; no `@media (pointer: coarse)`.
- **A4-P3-17** Icon-only buttons without accessible name: `#btn-close-search` (`index.html:951`),
  `#pill-quick-add` (`:518`).
- **A4-P3-18** Download poll outlives panel/view: `sections.js:1840-1853` — not stopped on module switch;
  with backend down the 1.5 s interval never terminates.
- **A4-P3-19** Toast hide timeouts never cleared (`app.js:1976-1979`) → a second toast is hidden early
  by the first toast's timer.
- **A4-P3-20** Single unescaped `innerHTML` sink: `app.js:967` inserts raw `l.language.toUpperCase()`
  (all other ~40 sinks escape — verified).
- **A4-P3-21** Unguarded `.toFixed()` on API star fields (`sections.js:1200-1201,1237`): one missing
  `magnitude`/`alt_deg` → TypeError on hover, NaN radii drawn.
- **A4-P3-22** Theme flash: `index.html:2` hardcodes `data-theme="light"` + label "LIGHT MODE" while
  `initTheme()` defaults `'dark'` (`app.js:125`) at DOMContentLoaded → light flash + extra render cycle.
- **A4-P3-23** Galaxy layout re-randomised every reload (`galaxy.js:250-259`): stored `node.x/y/z` from
  `/api/galaxy` never read → no spatial memory, graph disconnected from stored coordinates. (Same root
  as legacy A2-6.)

## Verified clean (searched, not assumed)

- SQL injection: all statements static + bound params; FTS5 `MATCH` receives the query as a bound
  parameter (`db.rs:412-428`), both query paths only emit quoted phrases — no FTS-language injection.
- Path traversal: `/maps/{filename}` + `DELETE` reject `/`, `\`, `..` (`maps.rs:47,155`); axum 0.8
  percent-decodes path params before the check; `/fonts` + SPA fallback use `ServeDir`.
- Command injection: `source`/bbox passed as discrete `Command::args` (`fetch.rs:423-434`), no shell.
- Fetch slot race: `begin()`/`finish()` under one mutex — no interleaving bug (hang/timeout is A4-P2-3).
- Off-by-one in note/backlink and chunker byte slicing (`notes.rs:17-33`, `content.rs:4-213`).
- No duplicated DOM ids, no `console.log` litter, no CORS wildcard, no secrets/`unsafe` in scope.
- ~39/40 `innerHTML` sinks correctly escaped (exception A4-P3-20).

## Not tested (stated, not implied)

Live server behavior (no instance probed this session), Ollama/LLM path, cross-tenant/authorization
(A01 structurally incomplete without a second account), A09 logging & alerting (not assessable from
outside), supply-chain integrity of vendored `web/vendor/*`, Windows/ARM portability, and real-network
PMTiles fetch. All P1–P3 findings are static-analysis **observations**, not PoC-validated vulns, except
A4-P0-3 which was reproduced by exact-algorithm replay.

## Suggested fix order (when authorized)

1. **A4-P0-3** — one-line divisor fix + regression test over Feb 26–28 (backend correctness, dated).
2. **A4-P0-1** — delete the duplicate `app.js` toolbar bindings (restores 4 controls).
3. **A4-P0-2** — reset `selectedCategory` in `showHierarchyRoot()`.
4. **A4-P1-1/P1-2** — stream `/maps` responses + RFC-compliant Range parsing (OOM/corruption class).
5. **A4-P1-3..P1-8** — frontend race/teardown/telemetry batch.
6. **A4-P2-1..P2-3** — fetch endpoint hardening (SSRF allowlist, bbox cap, job timeout) before any
   `0.0.0.0` deployment (A4-P2-6).

