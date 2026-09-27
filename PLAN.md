# PLAN.md — Mazzaroth galaxy repair

Scope: fix the four reported symptoms without rewriting the project.
Mode: audit-only so far — **S1–S5 are `[PENDING]`, nothing implemented.**
Companion ledger: `GATES.md` (G7–G11). Detail and evidence: `AUDIT.md`.

## Context

`cargo build` / `cargo test` are green (`AUDIT.md` §1). What fails is the galaxy:
`step_physics` collapses all 1097 stars onto the origin in ~20 s, so the browser
draws the 30 000 procedural dust particles instead of the database's spiral
(`AUDIT.md` §2). Rotation wiring exists but spins an empty centre (§3), and the
galaxy payload carries no `content`/`tags`/edges, so a tap renders placeholders (§4).

## Decision required before S1 (pick one, then I re-verify)

| # | Approach | Trade-off | Effort |
|---|---|---|---|
| **A (recommended)** | **DB-authoritative positions**: drop `step_physics` from the serve/render loop, serve `sync_celestial_bodies()` output, let the client's `galaxyGroup` do the motion | Shortest diff; star field is stable and identical every load; loses free-floating N-body drift | ~30 min |
| B | Keep the sim, give it a real orbit equilibrium (`v_t = √(GM/r)`, springs off or capped), write positions back to SQLite on settle | Keeps "living" physics; needs tuning so it cannot re-collapse; positions can drift from the importer's layout | 2–3 h |
| C | Keep the sim server-side but freeze it after N ticks (OpenViking pattern), render the frozen layout | Middle ground; still spends 50 % CPU while frozen unless the loop stops | 1 h |

A + the client-side rotation already in `web/index.html:683` satisfies every
reported symptom; B/C only matter if you want N-body drift as a feature.

## Stages

- [ ] **S1 — Stop the collapse; make the database build the star field**
  OWNS: `src/engine.rs`, `src/celestial/physics.rs`, `src/main.rs`
  Change: apply decision A (or B/C above) so `/api/memory/celestial` returns the
  stored spiral at any uptime; kill the 20 Hz thread if the sim leaves the loop.
  Also: replace the `O(links × bodies)` scan with an id→index map (`physics.rs:60-96`).
  Gate: **G7** (`GALAXY_ATTACHED`). Evidence now: `GALAXY_DETACHED 42.8 -> 1.5`.

- [ ] **S2 — Rotation that is visible**
  OWNS: `web/index.html`, `src/visualizer/app.rs`
  Change: star field must visibly rotate on load (group rotation independent of the
  camera-orbit toggle), the Orbit button must reflect its real state, and per-star
  orbital motion either uses `orbit_angle` (`physics.rs:37`, `body.rs:28`) or is
  removed from the HUD (`app.rs:128`). Stop `flyCameraTo` (`index.html:535-538`)
  pointing at space the group has rotated away from.
  Gates: **G8** (wiring, runnable), **G10** (visible, manual).

- [ ] **S3 — Tap a star → real data + its connections**
  OWNS: `src/celestial/body.rs`, `src/engine.rs`, `src/server/http.rs`, `web/index.html`
  Change: either embed `content`, `tags`, `created_at`, `last_accessed` and a
  per-star `links[] {target, relationship, weight}` in the galaxy payload, or add
  `GET /api/memory/node?id=` (data already exists: `GET /api/memory/nodes`, 562 KB).
  Then: inspector shows real content/tags/edge types, highlight the selected star,
  show "n of m" on the connection grid, fix `node-header` vs `.note-header`
  (`index.html:269` vs `:168`).
  Gate: **G9** (`STAR_DATA_PRESENT`), **G10** (manual).

- [ ] **S4 — Stop the 4-second rebuild churn**
  OWNS: `web/index.html`
  Change: diff-update `starPoints`/`constellationLines` instead of recreating them
  (`index.html:432-496`), don't wipe the cluster sidebar every poll (`:499-512`),
  poll only on a change the server reports (or at a lower rate).
  Gate: **G10** (manual: scroll and clicks survive 10 s) + payload size evidence in `AUDIT.md` §7.4.

- [ ] **S5 — Native GUI parity**
  OWNS: `src/visualizer/app.rs`
  Change: inspector shows content/tags/connections like the web; replace the
  per-frame `O(lines × bodies)` line lookup (`app.rs:164-167`) with a map.
  Gate: **G10** (manual).

- [ ] **S6 — Re-verify everything**
  OWNS: `GATES.md` (evidence lines only)
  Change: run G7–G11, paste real output into `EVIDENCE:`, re-measure `AUDIT.md` §7 numbers.
  Gate: **all of G7–G11 met or explicitly `ABANDON:`**.

## Scope boundaries

In scope: `src/**`, `web/index.html`, `GATES.md`, `PLAN.md`, `AUDIT.md`.
Out of scope: `bibles/`, `data/*.db` (never write to the live database), `tests/`
history in `GATES.md` M1–M6, anything outside `/home/xander/Documents/portfolio/mazzaroth/`.

## Open questions

- [ ] What exact error did your build print? Everything is green here (`AUDIT.md` §1) — paste the first 10 lines and I add a gate for it.
- [ ] Decision A, B, or C above?
- [ ] Web only, or must the egui GUI get the same inspector (S5)?

## Risks

- Changing `step_physics` changes `tests/physics_test.rs:14-15`, which currently
  *asserts* the collapse — that assertion must be rewritten in the same change or
  the suite goes red.
- Serving `content` inside the galaxy payload grows 677 KB → several MB; prefer the
  on-tap endpoint if that matters (S3 option 2).

## Notes

- 2026-09-27 — audit only; no source file touched. Evidence in `AUDIT.md` §7;
  gate G7 negative control verified (`GALAXY_DETACHED 42.8 -> 1.5`), gate G9
  negative control verified (`STAR_DATA_MISSING embedded=0/1097`).

---
*Created: 2026-09-27*
