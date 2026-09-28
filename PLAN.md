# PLAN.md — Mazzaroth Reshape: UX Stabilization, SBC/Offline, Encyclopedia & Astronomy

Mode: **S24–S28 [DONE] + fix-pass [DONE @ 6f0fb07]** — next stage **S29**. Companion ledger: `GATES.md`.

## Next action

Execute **Stage S29 (UX STABILIZATION)**: (1) section-switch state isolation, (2) sidebar discoverability
(path chips + per-step headers + filter-only search). Gates: **G34, G35**.

## 0. Audit verdict 2026-09-28 (this review, re-run by me)

1. Fixes verified: `currentCelestialData` = 0 hits, `openHud` resets reading pane, galaxy route wired,
   empty dirs gone, hardened G29 present; **G25 loop re-run: 25/25 `LEDGER_ALL_GREEN`**; build/test/clippy clean.
2. **Reported bug — section-switch overlap (root cause):** `switchSection` (`web/js/main.js:260`) never
   closes the HUD / `currentFocusNode` / hover tooltip / SVG selection overlay / breadcrumb buttons /
   drilldown position. Stale panels and filaments from the old section stay drawn over the new one.
3. **Reported bug — sidebar not user-friendly (root causes):** three competing headers (SECTOR DIRECTORY,
   STEP badge, cluster-header-text); no visible path ("eng › kjv › Genesis"); search box does double duty
   (filter vs Enter = FTS warp to arbitrary star); no hint that step 1 IS the language list; drilldown
   state persists across section switches (land mid-flow); reading pane discoverable only at step 4.

## 1. Stages

- **S29 — UX STABILIZATION** [PENDING] — OWNS: `web/js/main.js`, `web/index.html`, `web/css/base.css`.
  a) `switchSection` resets: closeHud, hide tooltip, clear `#selection-overlay-svg`, hide nav crumbs,
  `drilldownState = step 1`, clear search input. b) Sidebar: persistent path chips (`#drill-path`),
  per-step list header (`#drill-list-header`, e.g. "60 languages — pick one"), step-1 hint line,
  search = filter only (Enter selects first hit, FTS moves to its own quiet entry), back button always
  visible at step ≥ 2. Gates: **G34 SECTION_ISOLATION, G35 NAV_DISCOVERABILITY**.
- **S30 — OFFLINE & SBC** [PENDING] — OWNS: `web/vendor/`, `web/index.html`, `Cargo.toml`, `install.sh`.
  Vendor three.js r128 + OrbitControls + tween locally (currently CDN — breaks offline/SBC);
  `[profile.release] lto=true, codegen-units=1, strip=true`; scripture cache cap via `MAZZAROTH_CACHE`
  (baseline measured: binary 8.87 MB, RSS 9.4 MB); headless auto `--no-browser`; aarch64 build note.
  Gates: **G36 OFFLINE_ASSETS, G37 SBC_FOOTPRINT (binary ≤ 9 MB, RSS ≤ 64 MB)**.
- **S31 — ENCYCLOPEDIA SECTION** [PENDING] — OWNS: `encyclopedia/{src,data,web}`, `src/lib.rs`,
  `src/server/http.rs`, registry entry. Offline packs (JSON: domain/title/body — bible, biology, medicine,
  survival; user-provided + public-domain corpora). Reuse FTS via `memory/ingest` ids `encyclopedia:*`,
  reuse drilldown (Domain → Topic) + reading pane. Gate: **G38 ENCYCLOPEDIA_SECTION**.
- **S32 — ASTRONOMY SECTION** [PENDING] — OWNS: `astronomy/{src,data,web}`, registry entry.
  Data (planets, moons, asteroids vs comets, glossary) + card visuals: scale comparison, body bios,
  minor-body classifier. Pattern = constellation (JSON data + section mount + grid cards). Gate: **G39 ASTRONOMY_SECTION**.
- **S33 — GUI** [PENDING — awaiting user-provided spec] — contract = HTTP API (parity by API, not code).
  Gate: **G40 GUI_SURFACE** (placeholder).

## 2. Reconciliation vs this request

| Request | Status |
|---|---|
| Review applied fixes | DONE — verified, ledger 25/25 |
| Section-switch overlap | ROOT-CAUSED → S29a / G34 |
| Sidebar discoverability | ROOT-CAUSED → S29b / G35 |
| PLAN.md discipline (skill) | DONE — this file, live from here |
| SBC optimization | STRATEGIZED → S30 / G36–G37 (baseline measured) |
| Encyclopedia section | STRATEGIZED → S31 / G38 |
| Astronomy section | STRATEGIZED → S32 / G39 |
| GUI later | PLACEHOLDER → S33 / G40 |

## Scope boundaries

In scope: `src/**`, `galaxy/**`, `constellation/**`, `encyclopedia/**`, `astronomy/**`, `web/**`,
`Cargo.toml`, `install.sh`, `GATES.md`, `PLAN.md`. Out of scope: anything outside this repo.
