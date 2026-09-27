# AUDIT: galaxy ↔ database, rotation, star tap

Read-only audit. No source file was modified. Every claim below is either `file:line` in this repo or a command I ran in this session (outputs in §7).

Verdict, in one line each:

| # | Symptom you reported | Root cause | Severity |
|---|---|---|---|
| 1 | "failing to build it" | `cargo build`/`build --release`/`build --locked`/`cargo test` are all **green here** (§7.1). Build is not the failure — the *galaxy build* is: it collapses in ~20 s (§2) | Needs your exact error text |
| 2 | Galaxy detached from database | `step_physics()` pulls every body to the origin and never writes back; the DB spiral is dead data after ~20 s | **Blocker** |
| 3 | There should be rotation | Rotation wiring exists but only spins an empty centre; per-star orbital motion is computed then discarded (`orbit_angle` never applied) | High |
| 4 | Tap a star → its data + connections | `CelestialBody` has no `content`/`tags`/edges → inspector renders placeholders; all stars sit at one point so the raycast picks an arbitrary index | High |

---

## 1. Build

Measured this session, repo at commit `873dd0f`, clean tree:

- `cargo build` → `Finished dev profile`, 0 warnings
- `cargo build --release` → `Finished`
- `cargo build --locked` → `Finished` (Cargo.lock in sync)
- `cargo test` → 5/5 integration binaries pass + doc-tests (matches `GATES.md` M1–M6)

So the crate, both binaries (`mazzaroth`, `mazzaroth-gui`) and the web asset compile and run. `./target/debug/mazzaroth-gui` stays alive under XWayland; `/` serves HTTP 200; `/api/mcp` answers JSON-RPC.

Three non-`cargo` things can still look like "build failed", check in this order:

1. **CDN scripts** — `web/index.html:7-9` pulls three.js r128, OrbitControls, tween.js from cdnjs/jsdelivr. Offline or blocked ⇒ black canvas, JS never runs. All three returned HTTP 200 from here.
2. **DISPLAY** — `gui.sh:3` defaults `DISPLAY=:1`. On a native Wayland session with no XWayland socket, eframe exits before painting.
3. **Fresh clone with no `bibles/`** — `src/main.rs:45` skips import when the dir is missing ⇒ galaxy = 2 anchor nodes only.

If you saw a real `error[...]` from cargo, paste the first 10 lines; nothing in this tree reproduces it.

## 2. Galaxy detached from the database — blocker

### 2.1 The database is correct

```
nodes 1097 · links 1155
lang ring radius: min 120.3  max 562.5  mean 341.6
all-body radius:  mean 342.4  max 602.0
rows at origin:   2   (the two celestial anchors)
```

`src/corpus/importer.rs:74-101` writes a real logarithmic spiral into `pos_x/pos_y/pos_z`; `src/store/sqlite.rs:49-54` persists it. Import → `sync_celestial_bodies()` (`src/corpus/importer.rs:252`) loads it. Nothing is wrong on the write side.

### 2.2 The simulator destroys it in ~20 seconds

`src/main.rs:53-59` runs `step_physics(0.05)` at 20 Hz forever. `src/celestial/physics.rs:40-57` applies central gravity `G·M·m/r²` with **no equilibrium orbit**: the tangential term (`physics.rs:55-56`) is `orbit_speed · 15`, orders of magnitude below `√(GM/r)`, so every body falls inward; `damping 0.96` caps the terminal speed and the swarm settles at r ≈ 1.6.

Measured on a fresh server (`/api/memory/celestial`, sample every 3 s):

| elapsed | lang mean r | lang max r | all mean r | bodies with r<5 |
|---|---|---|---|---|
| 0.1 s | 82.7 | 213.4 | 304.0 | 1 |
| 6.2 s | 13.1 | 61.0 | 62.1 | 50 |
| 12.4 s | 2.3 | 16.0 | 13.5 | 594 |
| 18.5 s | 1.6 | 2.9 | 2.3 | **1002 / 1097** |
| 21.6 s | 1.7 | 3.1 | 1.7 | 1002 |

DB is untouched (still 2 rows at origin). Live positions are memory-only: `step_physics` (`src/engine.rs:190-202`) never calls `insert_node`, and `get_galaxy_state` (`src/engine.rs:204-213`) serves the in-memory vector. **The renderer is showing a simulation that has no relationship to the layout you stored.**

### 2.3 What the browser actually draws

- 1097 DB stars → 151 distinct positions at r < 3: one bright knot at the core.
- 1155 constellation lines → median length **15.1**, max **15.1** (DB says core→lang spans 120–562): the filaments are invisible, so `data.lines` looks empty.
- The only thing that reads as "a galaxy" is `createCosmicSpiralDust()` (`web/index.html:366-415`): 30 000 **procedural, non-DB** particles. That is the detachment — decoration is doing the job the database was supposed to do.

### 2.4 Two extra detach symptoms

- **Ingest teleports the whole galaxy.** `engine.rs:152` re-reads all 1097 nodes and replaces the live vector on every `ingest`, so "✨ Ignite Memory Star" (`web/index.html:649-666`) snaps every star back to the DB layout, then the collapse restarts.
- **`orbit_angle` is dead.** `physics.rs:37` advances it, `body.rs:28` exposes it, `app.rs:128` prints it — nothing ever sets `x/z` from it. There is no orbital motion, only free-fall.

### 2.5 Cost

Physics link springs are `for link in links { for body in bodies … }` (`physics.rs:60-96`): 1155 × 1097 ≈ **1.27 M string comparisons per tick**, 20 ticks/s ⇒ server measured at **50.9 % CPU** while idle. The egui renderer repeats the same `O(lines × bodies)` scan every frame (`src/visualizer/app.rs:164-167`).

### 2.6 Why no gate caught it

- `tests/physics_test.rs:14-15` **asserts the collapse**: `assert!(bodies[0].vx < 0.0, "Gravity must pull star towards galactic center")`.
- `tests/corpus_test.rs:14-16` checks the galaxy only immediately after import, before any physics tick.

## 3. Rotation

What exists:

| Surface | Behaviour | Anchor |
|---|---|---|
| Web group | `galaxyGroup.rotation.y += 0.0007` per frame, **gated on `controls.autoRotate`** | `web/index.html:683-685` |
| Web camera | `controls.autoRotate = true`, speed 0.25 | `web/index.html:320-321`, toggle `:668-670` |
| egui camera | `camera.rot_y += dt * 0.15` when `auto_rotate` | `src/visualizer/app.rs:50-52`, checkbox `:62` |

What is missing:

1. **Nothing visible rotates** — the DB stars are a single knot at the origin (§2.2), so group rotation moves only the decorative dust.
2. **No per-star orbit** — `orbit_angle`/`orbit_radius` never reach the position (§2.4), so stars do not travel around the core.
3. **One toggle drives both motions** — turning off camera auto-rotate also stops the group (`index.html:683`), and the button never changes its active state, so the UI can claim rotation is on while it is off.
4. **Rotation fights inspection** — `flyCameraTo` tweens to the star's world position at click time (`index.html:535-538`), but the group keeps turning, so the camera settles pointing at empty space within a couple of seconds.

Reference: obsidian-galaxy ships rotation as a first-class setting pair — `autoRotate` + `autoRotateSpeed` on the orbit controls with a dedicated UI section (`obsidian-galaxy/src/render.ts:286-295`, `src/controls-panel.ts:183-202`).

## 4. Tap a star → data + connections

### 4.1 The payload has no data

`CelestialBody` (`src/celestial/body.rs:13-30`) = id, label, tier, position, velocity, mass, luminosity, radius, orbit, colour. Live check: **0 / 1097 bodies carry `content` or `tags`.** The data exists one endpoint away — `GET /api/memory/nodes` returns all 1097 nodes with content/tags (562 KB) — but the galaxy route (`src/server/http.rs:36`) never includes it.

Consequence in `showInspector`:

- `index.html:570` → `body.content || "Cognitive celestial memory star active…"` → every star shows the same sentence.
- `index.html:574` → `body.tags || ['memory','second-brain']` → every star shows the same two fake tags.
- `created_at` / `last_accessed` / `access_count` / `relationship` / `weight` are not in the payload at all, so "recency" and "mass" (`:567-568`) are the only real numbers.

### 4.2 Connections are computed but not shown as data

`index.html:582-596` builds the connection grid from `galaxyData.lines` — structurally correct (1155 links resolve) but:

- cards show `id.split(':')[1]` only (`:593`) — no relationship type, no weight;
- the list is capped at 8 with no count ("showing 8 of 47");
- when the field has collapsed, every card points at the same knot, so the graph *looks* empty even though 1155 edges exist.

### 4.3 The click itself is unreliable

- `index.html:341-355`: raycast → `intersects[0].index` → `galaxyData.bodies[idx]`. Mapping is valid, but all points overlap at the origin within `raycaster.params.Points.threshold = 14` (`:338`) ⇒ the index you get is essentially arbitrary.
- No persistent highlight of the selected star — only the hover tooltip (`:223-238` is egui; the web side has no selection visual at all).
- `web/index.html:269` uses class `node-header`, CSS defines `.note-header` (`:168`) — the inspector header row is unstyled.

### 4.4 The native GUI is worse

`src/visualizer/app.rs:120-132` shows label, tier, mass, luminosity, position, orbit radius. No content, no tags, no connection list anywhere in the GUI, and `selected_body` is a frozen snapshot taken at click time while the body keeps moving.

## 5. Secondary defects (same files, fix while you are in there)

1. **Full rebuild every 4 s** — `fetchGalaxy` (`index.html:358-359`) discards and recreates `starPoints`, `constellationLines` and the whole cluster sidebar (`:499-512`). Sidebar scroll position resets and in-flight clicks are dropped every 4 seconds; 677 KB of JSON per poll.
2. **`O(n·m)` scans** — `physics.rs:60-96` (1.27 M comparisons/tick) and `app.rs:164-167`. An id→index `HashMap` built once per tick/frame is the whole fix.
3. **Decay never runs** — `CognitiveDecayEngine` is constructed (`engine.rs:30,43`) and tested, but `apply_temporal_decay` is called nowhere in `src/`. The "🔥 Recency" glow is static `activation` from the DB, not recency.
4. **Hard-coded HUD copy** — `index.html:245` says "60 Clusters" before the first fetch (fine) but `stat-links` label says "Constellations" while the value is links (`:246`).
5. **Payload weight** — 677 KB / 4 s for a field that changes at ingest/search time; no `limit`/`offset` anywhere (memanto: `memory_read_service.py:303-315`; TencentDB: `v2-router.ts:1113-1142`).

## 6. Reference systems — adopt / don't adopt

Analyzed: `obsidian-galaxy`, `memanto`, `OpenViking`, `TencentDB-Agent-Memory`.

**Adopt**

1. **Renderer owns positions; DB owns topology** — one SQL pass yields nodes + edges + degree, coordinates are derived, never simulated into a corner (TencentDB `MemoryKnowledge/src/engines/wiki/index-db.ts:97-101` + `manager.ts:414-434`; obsidian-galaxy `src/graph-data.ts:40-92`).
2. **Stabilize once, then freeze** — OpenViking disables physics after the first settling pass (`openviking/session/memory/graph_view.py:644-647`). Exactly the discipline Mazzaroth's 20 Hz free-fall violates.
3. **Edges as first-class objects in the payload** — `source, target, link_type, weight, description` rendered in the inspector (`graph_view.py:139-153`, `:543-549`).
4. **Per-node `related[]` (in/out/both, degree-sorted, capped)** for the tap panel (`manager.ts:444-476`) — cheaper than scanning all 1155 lines client-side.
5. **Rotation as a settings pair with its own UI section** (`render.ts:286-295`, `controls-panel.ts:183-202`).

**Reject**

1. obsidian-galaxy's "reheat the force sim on every change, never persist positions" (`render.ts:302-333`) — reproduces this exact detachment bug.
2. TencentDB's `Math.random()` node seeding (`KnowledgeGraph.tsx:122`) — star field reshuffles every reload.
3. OpenViking's `content_full` for every node with no paging (`graph_view.py:96,135`) — 1097 full bible excerpts per poll does not scale; fetch content on tap instead.
4. obsidian-galaxy's "open the source file" as its detail view (`graph-view.ts:152-156`) — Mazzaroth needs an in-app inspector.

## 7. Evidence log

```bash
# 7.1 build (all green, 0 warnings)
cargo build            # Finished dev profile …
cargo build --release  # Finished
cargo build --locked   # Finished
cargo test             # 5/5 binaries + doc-tests ok

# 7.2 database layout
sqlite3 data/mazzaroth.db "select count(*) from nodes; select count(*) from links;"
# 1097 / 1155
sqlite3 data/mazzaroth.db "select round(min(...)),round(max(...)),round(avg(...)) from nodes where id like 'celestial:lang:%';"
# 120.3 | 562.5 | 341.6

# 7.3 collapse timeline (fresh server, /api/memory/celestial every 3 s)
# 0.1s lang_mean 82.7  →  18.5s lang_mean 1.6, 1002/1097 bodies at r<5

# 7.4 payload facts
# bodies 1097, lines 1155, "content" present on 0 bodies, "tags" present on 0 bodies
# response size 676 708 bytes; /api/memory/nodes 562 360 bytes
# constellation line length: min 0.00 median 15.07 max 15.10 (DB expects 120-562)
# distinct rounded positions: 151 of 1097

# 7.5 runtime cost
ps -o pid,%cpu -C mazzaroth   # 50.9 % CPU while idle

# 7.6 gate self-test (each CHECK in GATES.md executed as written, 2026-09-27)
# G7  rc=0  no token     -> UNMET   GALAXY_DETACHED lang_mean 42.8 -> 1.5
# G8  rc=0  token        -> MET     ROTATION_WIRING_PRESENT
# G9  rc=0  no token     -> UNMET   STAR_DATA_MISSING embedded=0/1097 and no /api/memory/node
# G10 manual, no CHECK
# G11 rc=0  token        -> MET     Finished `dev` profile …
```

## 8. Resolution map

Each symptom → the stage in `PLAN.md` that closes it → the gate in `GATES.md` that proves it:

| Symptom | Stage | Gate |
|---|---|---|
| Galaxy not built from DB | S1 | G7 `GALAXY_ATTACHED` |
| Rotation | S2 | G8 (wiring, runnable) + G10 (visible, manual) |
| Tap → data + connections | S3 | G9 `STAR_DATA_PRESENT` + G10 (manual) |
| Build | — | G11 (already met; re-run if your error returns) |
