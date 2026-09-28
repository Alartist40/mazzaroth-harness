# PLAN.md — Mazzaroth Reshape & Modular Section Architecture

Scope: Modular section architecture, single-surface web visualizer, UI foundation cleaning, file-backed scripture API, 4-step hierarchical navigator & reading pane.
Mode: **S28 complete** — Ledger truth verified (G7–G11, G13–G17, G20, G23, G24, G26–G30, G32, G33 MET; G12, G18, G19, G21, G22 ABANDON). Companion ledger: `GATES.md`.

## Next action

Run `mazzaroth` (or `./run.sh`) to interact with the 3D particle spiral galaxy database, 4-step hierarchical scripture navigator, verse reading pane, and 32 classical/zodiac constellations.

## 1. Done and verified (re-run, not copied)

| Gate | CHECK re-run | Result |
|---|---|---|
| M1–M6 | Crate compilation, cognitive decay, physics projection, SQLite FTS5, MCP server, corpus ingestion | **MET** |
| G7 | 24 s stability check | `lang_mean=341.6` constant at t=6/12/18/24 s → **MET** |
| G8 | rotation wiring grep | `ROTATION_WIRING_PRESENT` → **MET** |
| G9 | endpoint probe | content+tags+links returned (core: 60 links) → **MET** |
| G10 | manual visual pass | low-CPU 30 FPS rotation, real data inspector, and breadcrumbs → **MET** |
| G11 | `cargo build --locked --all-targets`, `cargo test` | 0 warnings, 5/5 tests pass → **MET** |
| G12 | native GUI parity | **ABANDON** (single-surface decision) |
| G13 | spiral geometric expansion & 0.003 twist alignment | `SPIRAL_ALIGNED dev<=0.05rad gaps_increase bridge_max<=200` → **MET** |
| G14 | star point scaling & depthWrite:false | `STAR_SCALE_OK` → **MET** |
| G15 | radial brightness & dynamic luminosity modulation | `RADIAL_GLOW_OK` → **MET** |
| G16 | on-demand screen-space curved Bezier filaments | `ON_DEMAND_FILAMENTS_OK` → **MET** |
| G17 | centre & rim star readability | `READABILITY_OK` → **MET** |
| G18 | native GUI star scale | **ABANDON** (single-surface decision) |
| G19 | native GUI 6k spiral dust | **ABANDON** (single-surface decision) |
| G20 | sections registry backend and frontend switching | `SECTIONS_REGISTRY_OK` → **MET** |
| G21 | spiral ribbons | **ABANDON** (superseded by clean 3D dust) |
| G22 | planetary schematic | **ABANDON** (superseded by clean HUD + square reticle) |
| G23 | full 12 Zodiac signs + 20 asterisms catalog | `FULL_ZODIAC_CATALOG_OK` → **MET** |
| G24 | core fallback guarded strictly to galaxy section | `CORE_FALLBACK_GATED` → **MET** |
| G26 | section layout & ServeDir static serving | `SECTION_LAYOUT_OK` → **MET** |
| G27 | UI clean, emoji removal, scrollbars, inline style elimination | `UI_CLEAN_OK` → **MET** |
| G28 | file-backed scripture loader & endpoints | `SCRIPTURE_OK` → **MET** |
| G29 | 4-step drill-down navigator & live filtering | `NAV_DRILLDOWN_OK` → **MET** |
| G30 | HUD scripture reading pane & verse numbers | `READING_PANE_OK` → **MET** |
| G32 | GUI & eframe removal | `GUI_REMOVED_OK` → **MET** |
| G33 | install.sh one-command launcher | `ONE_COMMAND_OK` → **MET** |
| G25 | automated ledger verification loop | `LEDGER_ALL_GREEN` → **MET** |

## 2. Completed Steps

- **S24 — RESTRUCTURE.** [DONE] Single crate with `#[path]` module wiring (`galaxy/`, `constellation/`), static serving via `tower_http::services::ServeDir`, and web split into `web/index.html`, `web/css/base.css`, `web/js/main.js`. Gate G26 verified.
- **S25 — UI FOUNDATION.** [DONE] Removed emojis from chrome and dynamically from DB labels via `cleanLabel`, added global thin scrollbar rules, applied 2-accent design tokens, removed inline styles, preserved square reticles. Gate G27 verified.
- **S26 — SCRIPTURE API.** [DONE] File-backed scripture reader with LRU caching, `/api/scripture/meta`, `/api/scripture`, `/api/scripture/languages`, `/api/scripture/versions`. Gate G28 verified.
- **S27 — UX NAVIGATOR & READING PANE.** [DONE] 4-step drill-down (Language → Version → Book → Chapter), type-ahead search filtering, verse-by-verse reading pane with prev/next chapter navigation and Esc step-back. Gates G29, G30 verified.
- **S28 — CLOSEOUT & SINGLE LAUNCHER.** [DONE] Removed desktop GUI + eframe dep, added `install.sh` for one-command execution (`mazzaroth`), updated documentation and verified full G25 ledger loop. Gates G32, G33 verified.

## Scope boundaries

In scope: `src/**`, `galaxy/**`, `constellation/**`, `web/**`, `GATES.md`, `PLAN.md`, `README.md`.
Out of scope: anything outside `/home/xander/Documents/portfolio/mazzaroth/`.
