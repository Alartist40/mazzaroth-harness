# PLAN.md — Mazzaroth Reshape & Modular Section Architecture

Scope: Modular section architecture, UI foundation cleaning, DB bulk scripture ingestion & reader, 4-step hierarchical navigator.
Mode: **S24 complete** — Section layout and ServeDir active (G7–G20, G23–G26 MET, G21/G22 ABANDON). Companion ledger: `GATES.md`.

## Next action

Execute **Stage S25 (UI FOUNDATION)**: remove 21 emojis from chrome, add scrollbar CSS, eliminate inline styles, refine theme tokens (#05070d bg, cyan #57d7ff & violet #a78bfa accents), and preserve corner reticles.

## 1. Done and verified (re-run, not copied)

| Gate | CHECK re-run | Result |
|---|---|---|
| G7 | 24 s stability check | `lang_mean=341.6` constant at t=6/12/18/24 s → **MET** |
| G8 | rotation wiring grep | `ROTATION_WIRING_PRESENT` → **MET** |
| G9 | endpoint probe | content+tags+links returned (core: 60 links) → **MET** |
| G11 | `cargo build --locked --all-targets` after forced rebuild, `cargo test`, `cargo clippy` | 0 warnings, 5/5 tests pass, clippy clean → **MET** |
| G12 | `grep -c step_physics src/visualizer/app.rs == 0` | `NATIVE_GUI_PARITY_VERIFIED` → **MET** |
| G13 | spiral geometric expansion & 0.003 twist alignment | `SPIRAL_ALIGNED dev<=0.05rad gaps_increase bridge_max<=200` → **MET** |
| G14 | star point scaling & depthWrite:false | `STAR_SCALE_OK` → **MET** |
| G15 | radial brightness & dynamic luminosity modulation | `RADIAL_GLOW_OK` → **MET** |
| G16 | on-demand screen-space curved Bezier filaments | `ON_DEMAND_FILAMENTS_OK` → **MET** |
| G17 | centre & rim star readability | `READABILITY_OK` → **MET** |
| G18 | native GUI star scale and corona capping | `GUI_SCALE corona<=1.8 core_opaque=0` → **MET** |
| G19 | native GUI 6k spiral dust backdrop and radial glow | `GUI_LOOK_OK` → **MET** |
| G20 | sections registry backend and frontend switching | `SECTIONS_REGISTRY_OK` → **MET** |
| G21 | spiral ribbons (superseded by clean 3D dust) | **ABANDON** |
| G22 | planetary schematic (superseded by clean HUD + square reticle) | **ABANDON** |
| G23 | full 12 Zodiac signs + 20 asterisms catalog | `FULL_ZODIAC_CATALOG_OK` → **MET** |
| G24 | core fallback guarded strictly to galaxy section | `CORE_FALLBACK_GATED` → **MET** |
| G26 | section layout & ServeDir static serving | `SECTION_LAYOUT_OK` → **MET** |
| G25 | automated ledger verification loop | `LEDGER_ALL_GREEN` → **MET** |

## 2. Completed Steps

- **S24 — RESTRUCTURE.** [DONE] Single crate with `#[path]` module wiring (`galaxy/`, `constellation/`), static serving via `tower_http::services::ServeDir`, and web split into `web/index.html`, `web/css/base.css`, `web/js/main.js`. Gate G26 verified.

## 3. Stages in Progress

- **S25 — UI FOUNDATION.** [PENDING] OWNS: `web/css/base.css`, `web/index.html`, `web/js/main.js`. Strip emoji from chrome, add global thin scrollbar rules, apply 2-accent design tokens, remove inline styles, keep corner brackets. Gate: G27.
- **S26 — SCRIPTURE DB BULK IMPORT & API.** [PENDING] OWNS: `src/store/`, `src/server/http.rs`, `galaxy/src/`. Create scripture tables in SQLite, batch import engine from `galaxy/data/bibles/`, expose `/api/scripture/meta` and `/api/scripture`. Gate: G28.
- **S27 — UX NAVIGATOR & READING PANE.** [PENDING] OWNS: `web/js/main.js`, `web/index.html`, `web/css/base.css`. 4-step drill-down (Language -> Version -> Book -> Chapter), filtered search, reading pane with verse numbers and prev/next. Gates: G29, G30.
- **S28 — CLOSEOUT & G31 WALKTHROUGH.** [PENDING] Ledger re-verification, walkthrough artifact, final check. Gate: G31.

## Scope boundaries

In scope: `src/**`, `galaxy/**`, `constellation/**`, `web/**`, `GATES.md`, `PLAN.md`.
Out of scope: anything outside `/home/xander/Documents/portfolio/mazzaroth/`.
