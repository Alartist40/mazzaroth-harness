# AGENTS.md — Mazzaroth Project Conventions & Permissions

## 1. Project Overview

Mazzaroth is a sovereign, local-first cognitive memory daemon, MCP server, and 3D celestial particle visualizer built in Rust (Axum + SQLite) with a vanilla Three.js frontend.

### Repository Architecture

- `crates/`: Rust workspace — `core` (SQLite FTS5 persistence, config, notes, provenance, PMTiles reader, closed-form offline astrometry engine `sky.rs`, scripture/constellation loaders), `server` (Axum HTTP API: `/api/status`, `/galaxy`, `/search`, `/read`, `/api/tree`, `/api/sky`, `/ask`, `/maps`, `/api/maps/fetch` start+status, `DELETE /api/maps/{file}`, shared fetch engine `fetch.rs` → `maps/*.pmtiles`, ...), `cli` (`mazzaroth` binary: serve, ingest, doctor, optimize, maps fetch/list — fetch delegates to `server::fetch`).
- `web/`: Single-page frontend — `index.html` (5-zone UI, 5 system section workspaces: 01 Galaxy, 02 Constellations, 03 Librarian, 04 Maps, 05 Sky Deck), `css/app.css` (theme tokens, breadcrumbs, hierarchy cards, sky deck HUD), `js/api.js` (API contract incl. sky projection, knowledge tree, fetch/delete map calls), `js/app.js` (5-module switcher, deep reading deck hierarchy explorer with breadcrumbs, ⌘K, telemetry), `js/galaxy.js` (Three.js spiral), `js/sections.js` (constellations, live astrometry Sky Deck with alt/az dome projection, MapLibre map: schema-aware overview/street styles + region switcher + zoom/download tool buttons + 62-country download panel with chip delete, librarian), `vendor/` (offline three, maplibre-gl, pmtiles, tailwind), `fonts/Noto Sans Regular/` (complete 256-range Klokantech Noto Sans CJK glyph PBFs, 30.4 MB — kana/kanji/Latin; `/fonts` is a strict sub-service: missing range → 404, never the SPA index.html).
- `data/`: Live database `data/mazzaroth.db` (CLI `--db` default; nodes, links, constellations, documents, notes); `data/bin/pmtiles` (auto-downloaded official extract binary, gitignored).
- `maps/`: Offline vector tiles served at `/maps/{filename}` (gitignored, regenerable): `world.pmtiles` overview (tools/gen-map-tiles.py) + street packs from `mazzaroth maps fetch <bbox>` **or the in-app ↓DOWNLOAD tool** (same `POST /api/maps/fetch` job; e.g. `japan.pmtiles` 1.59 GB z14).
- `content/`: Seed corpus JSON (`content/pd-demo/`) consumed by `mazzaroth ingest`.
- `galaxy/data/bibles/`, `constellation/data/`: Raw ingestion assets only (JSON data; loader code lives in `crates/core`).
- `tools/`: Standalone ingestion utility scripts.
- `docs/`: Reference docs — `SCHEMA.md`, `SOURCING.md`, `AUDIT.md`, `contract-probes.txt`.
- Root harness: `PLAN.md`, `GATES.md`, `IMPLEMENT.md`, `README.md`, `AGENTS.md`, `install.sh`.

## 2. Tool Permissions & Rules

### Allowed (Auto)
- Reading, searching, analyzing all files in `crates/`, `web/`, `docs/`, `content/`, `tools/`, `galaxy/data/`, `constellation/data/`.
- Running cargo commands (`cargo check`, `cargo test`, `cargo build`).
- Updating documentation and gate ledgers (`PLAN.md`, `GATES.md`, `IMPLEMENT.md`, `docs/AUDIT.md`, `README.md`).

### Restricted / Ask First
- Destructive operations (`rm -rf`, force resets, schema deletions).
- Modifying core database schema without backward-compatible migrations.
- Adding heavy external runtime dependencies when stdlib/minimal code suffices.

### Not Allowed
- Reintroducing native desktop GUI crates (`eframe`, `egui`, `glow`, `winit`). The architecture is strictly committed to the single unified web visualizer surface.
- Adding unrequested abstractions, speculative framework bloat, or complex build tooling.

## 3. Engineering Disciplines & Verification

- **Harness Engineering**: `PLAN.md`, `GATES.md`, `IMPLEMENT.md` must be kept live throughout tasks.
- **Completion Gates**: Verify every outcome against executable `CHECK:` commands and real `EXPECT:` tokens before checking boxes.
- **Ladder of Minimal Code**: YAGNI → Reuse existing pattern → Stdlib → Native feature → Minimal code.
- **Escalation Rule**: If blocked outside permitted scope or facing architectural ambiguity, stop and describe the blocker clearly.
