# PRD — Offline Librarian Box

**Working title:** *Librarian Box* (rename freely — nothing in this doc depends on the name)
**Version:** 1.0 draft · **Status:** Approved scope, ready to build
**Source repos to mine for code:** [Paraclea](https://github.com/Alartist40/paraclea) (ingestion, retrieval, memory) · [Agentage Galaxy](https://github.com/agentage/obsidian-galaxy) (galaxy UI concepts) · Reference competitor: [Project NOMAD](https://github.com/Crosstalk-Solutions/project-nomad)

---

## 1. Product Vision

> **An offline librarian in a box.** A single Rust binary that runs on any ARM64 SBC, serves a curated, legally-clean knowledge library through a beautiful web UI, and answers questions about that library with a small local language model acting as a librarian. One binary, no Docker, no cloud.

You carry it on a hike, lose the internet, and it still works: read books, ask the librarian where something is, open offline maps, take field notes.

**Why this exists (vs. NOMAD):** NOMAD orchestrates ~8 Docker containers and recommends a 32 GB GPU machine. This product runs on a 4 GB Orange Pi. NOMAD ships raw quantity (all of Wikipedia); we ship **curation + provenance**: every document has a verified source, license, and date, and the librarian always shows them.

---

## 2. Goals & Non-Goals

### Goals (v1)
1. Single Rust binary + embedded web UI, reachable at `http://device:8080`.
2. Runs on 4 GB RAM SBCs (Raspberry Pi 4 floor); full AI tier on 8 GB+ (Orange Pi 5 primary).
3. Curated JSON library with **mandatory provenance metadata** enforced at ingestion.
4. Librarian AI (Ministral 3B via any Ollama-compatible server) that retrieves from the library and **cites sources with dates**.
5. Galaxy UI (your separate HTML system) visualizing the library as a navigable 3D/2D graph.
6. Offline maps served from single-file `.pmtiles`.
7. Field notes stored locally, linked to library passages.
8. Everything usable in `tiny` mode (retrieval + reading, no LLM).

### Non-Goals (v1 — resist these)
- ❌ Wikipedia/ZIM dumps, Kiwix, Kolibri, education platforms
- ❌ Docker, container catalog, app store, auto-updater, benchmarks/leaderboard
- ❌ Multi-user accounts, auth systems (single-user personal box)
- ❌ Vector database server (Qdrant) — v1 uses SQLite FTS5 only
- ❌ Embedded inference engine — Ollama (or any OpenAI-compatible server) stays an external, optional dependency
- ❌ Mesh networking (exists in Paraclea; port later if wanted)
- ❌ TTS in v1 core — it is Milestone 8, the first stretch goal

---

## 3. Hardware Profiles

| Profile | Hardware example | What runs |
|---|---|---|
| `tiny` | Pi 4 4GB, Pi Zero 2 W | Library + search + reader + maps + notes. No LLM. |
| `standard` | Orange Pi 5 8GB, Pi 5 8GB | Everything + Ministral 3B Q4 (~2.5 GB RAM), 2–4k context. |
| `full` | 16GB+ / desktop | Everything + larger context, optional bigger model. |

**Design rules:**
- One `profile` field in config; the app degrades, never crashes: if the LLM endpoint is unreachable, the UI shows "Librarian offline — search still works."
- The binary must never *require* the LLM to start. Retrieval is the product; AI is the assistant.
- RAM budget for `standard`: Rust server < 200 MB, FTS5 index < 500 MB, model ~2.5 GB, OS ~800 MB. Leaves headroom.
- Ollama runs on CPU. Do **not** plan around the RK3588 NPU (experimental toolchain, per-model effort, maintenance trap). CPU Q4 on 8 cores of an Orange Pi 5 gives usable 3B speeds.

---

## 4. Architecture

### 4.1 Monorepo layout

```
librarian-box/
├── Cargo.toml                 # workspace
├── crates/
│   ├── core/                  # domain: content, index, retrieval, provenance, config
│   ├── server/                # Axum HTTP server + static UI + API routes
│   └── cli/                   # binary: `librarian serve|ingest|doctor|reindex`
├── ui/                        # your separate HTML galaxy system (built here)
│   ├── src/                   # vanilla JS or your framework of choice
│   └── dist/                  # built output, embedded into the binary via include_dir / rust-embed
├── content/                   # JSON files ready to ingest (gitignored except ./content/pd-demo/)
│   └── pd-demo/               # small public-domain sample corpus, shipped with the repo
├── tools/
│   ├── ingest-gutenberg.rs    # Gutenberg text → content JSON
│   ├── ingest-fm.rs           # US Army field manual → content JSON
│   └── ingest-bible.rs        # public-domain translations only
└── docs/
    ├── SOURCING.md            # where to get legal content
    └── SCHEMA.md              # the JSON + DB schemas below, kept in sync
```

**Key decision:** the UI is **not** part of the Rust binary at build time conceptually — it's your separate HTML project, embedded as static assets at release (`rust-embed`). Develop it live with `librarian serve --dev-ui ./ui/src` serving the workspace directly.

### 4.2 Request flow

```
Browser ──HTTP──> Axum server
                 ├─ /api/search      → SQLite FTS5
                 ├─ /api/read/:doc   → content store
                 ├─ /api/ask         → retrieval → prompt assembly → Ollama → streamed answer
                 ├─ /api/galaxy      → graph JSON (nodes = docs, edges = links/tags)
                 ├─ /api/notes       → SQLite notes
                 └─ /maps/:region.pmtiles → static range-serving of pmtiles file
```

### 4.3 Dependencies (keep this list short)

| Need | Crate | Why |
|---|---|---|
| HTTP server | `axum` | proven, async, Paraclea-gui already uses it |
| SQLite | `rusqlite` (+bundled) | zero external daemon; FTS5 built in |
| Static assets | `rust-embed` | UI compiled into the binary |
| LLM client | `reqwest` | talk to Ollama/OpenAI-compatible endpoint; stream SSE |
| JSON | `serde` + `serde_json` | obvious |
| Config | `figment` or plain TOML | profiles |
| pmtiles | `pmtiles` crate or hand-rolled range reads | single-file maps |

Anything else needs a written justification in the PR. **No** Qdrant, **no** Docker, **no** embedders in v1.

---

## 5. Content Model

### 5.1 Content JSON schema (per book/document)

```json
{
  "id": "fm-21-76-survival",
  "title": "US Army Survival Manual FM 21-76",
  "category": "survival",
  "language": "en",
  "provenance": {
    "source": "https://... (or 'US Government Printing Office')",
    "publisher": "US Department of the Army",
    "license": "public-domain",
    "license_url": null,
    "retrieved_date": "2026-09-28",
    "notes": "1992 edition, digitized by ..."
  },
  "structure": [
    {
      "id": "ch-01",
      "title": "Chapter 1: Introduction",
      "sections": [
        {
          "id": "ch-01-s-02",
          "title": "Requirements for Survival",
          "text": "..."
        }
      ]
    }
  ]
}
```

**Ingestion MUST reject any file missing or malformed `provenance`.** Non-negotiable. This is the gate that prevents the copyright problem from ever recurring. Valid `license` values: `public-domain`, `cc-by`, `cc-by-sa`, `cc0`, `gutenberg` (their PD terms), `personal-use-only` (imported, but flagged; excluded if you ever publish a corpus).

### 5.2 SQLite schema (built at ingest time)

```sql
CREATE TABLE documents (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  category TEXT NOT NULL,
  language TEXT NOT NULL,
  license TEXT NOT NULL,
  source TEXT NOT NULL,
  retrieved_date TEXT NOT NULL,
  personal_use_only INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE chunks (            -- retrieval unit ≈ one section, or ~800-token slices
  id INTEGER PRIMARY KEY,
  doc_id TEXT NOT NULL REFERENCES documents(id),
  section_id TEXT NOT NULL,
  title_path TEXT NOT NULL,      -- "FM 21-76 > Ch 1 > Requirements for Survival"
  text TEXT NOT NULL
);
CREATE VIRTUAL TABLE chunks_fts USING fts5(
  text, title_path, doc_title,
  content='', contentless_delete=1   -- external-content FTS to avoid double storage
);
```

Plus `notes` table for Milestone 6 and an `ingest_log` (file hash, timestamp, warnings) so re-ingestion is idempotent.

### 5.3 Legal sourcing catalog (starter)

| Content | License | Source |
|---|---|---|
| KJV, WEB, ASV 1901, YLT, Geneva 1599, Brenton LXX | public domain | various JSON exporters; **verify per translation, ship only these** |
| US Army FM 21-76 / FM 3-05.70 (survival), FM 4-25.11 (first aid) | public domain (US Gov) | archive.org / govinfo.gov |
| Other USG manuals: USDA plant guides, FEMA guides, NOAA weather | public domain | agency sites |
| Project Gutenberg books | PD / Gutenberg license | gutenberg.org bulk text |
| MedlinePlus health topics | public domain (NLM) | NLM dumps/API |
| WikEM / medical wiki dumps | CC BY-SA (attribution required) | kiwix.org library |
| Country/regional maps | ODbL (OpenStreetMap) | protomaps.com builds |
| HYG Database v4.2 star catalog (~120k stars, RA/Dec/magnitude/constellation field) | CC BY-SA | github.com/astronexus/HYG-Database (single CSV; filter to magnitude ≤ 6.5 for a naked-eye subset) |
| Yale Bright Star Catalog, BSC5P (9,110 naked-eye stars) | freely distributed | Harvard CDS archive (tdc-www.harvard.edu/catalogs/bsc5.html) |
| Constellation lines, boundaries & star lore (Western, Chinese, Arabic, Navajo, Inuit…) | mostly CC BY-SA — **verify per culture** before shipping | github.com/Stellarium/stellarium-skycultures |
| Pre-cleaned Western asterism line data | open (MIT) | github.com/eleanorlutz/western_constellations_atlas_of_space |
| OpenNGC deep-sky objects (optional) | MIT | github.com/mattiaverga/OpenNGC |
| *Where There Is No Doctor* | free personal use, **NOT redistributable** | hesperian.org — ingest for yourself, never ship |

**Action item:** the 160-translation Bible data currently vendored in Paraclea is tainted — most translations (NIV, ESV, NKJV, etc.) are copyrighted. Rebuild your Bible corpus from the 6–8 public-domain translations only. Before ingesting anything you "found online," open the source repo and check it has a license that covers the **data** (code MIT ≠ data licensed).

---

## 6. The Librarian (AI design)

### 6.1 Behavior contract
1. Every answer is grounded: the server retrieves chunks (FTS5, top-k 5–8), assembles the prompt, and the model answers **only** from that context.
2. Every answer displays, under the text: source title, license, **retrieved/source date**. For medical content the date is mandatory and prominent — old books are confidently wrong; provenance display is your safety mechanism, not the model.
3. If retrieval returns nothing relevant: the librarian says "I don't have that in the library" — it never improvises survival or medical advice. This is a prompt rule **and** a UI affordance (answers outside context get flagged).
4. Escapes: the user can always open the retrieved chunks directly ("Read the passages").

### 6.2 Prompt skeleton

```
System: You are the Librarian of an offline collection of books.
Answer ONLY from the provided passages. If the passages do not contain
the answer, say so. Quote precisely. Never invent procedures.
User: [question]
Context:
[1] (FM 21-76, 1992, public domain) Ch 3 > Fire: "..."
[2] ...
```

### 6.3 Model config
- Default: `ministral-3b` via Ollama (`http://localhost:11434`), Q4 quantization.
- Endpoint is configurable to any OpenAI-compatible server (LM Studio, llama.cpp server) — same client code, two wire formats.
- Context budget: 2k for `standard`, 4k+ for `full`. Top-k and max chunk length derive from the profile.

---

## 7. Web UI (your HTML galaxy system)

Views, in priority order:
1. **Reader** — book/document browsing (category → book → chapter → section). Provenance footer on every document. This ships **first**; it's the product without any AI.
2. **Search** — FTS5 query box with highlighted hits and title-path breadcrumbs.
3. **Librarian** — chat with streaming answers, citations block, "read passages" links.
4. **Galaxy** — graph of the library (nodes = documents/sections, edges = category + explicit links; clusters by category). Requirements: **2D fallback is day-one** (a phone hitting an SBC should not require WebGL); nodes must link into the Reader; search must spotlight nodes. Port interaction ideas from Agentage Galaxy (folders→clusters, click-to-spotlight), not its code (it's an Obsidian plugin).
5. **Maps** — MapLibre + pmtiles viewer; list of installed regions.
6. **Notes** — markdown editor; each note can link `[[doc-id#section-id]]`, and linked notes appear as galaxy nodes.

---

## 8. Milestones (build in this order — each one is shippable)

### M0 — Skeleton (week 1)
- [ ] Workspace builds: `crates/{core,server,cli}` + empty `ui/` + config with profiles.
- [ ] `librarian serve` runs Axum, serves a "hello" static page on :8080.
- [ ] CI: `cargo test` + `clippy --deny warnings` on linux-arm64 and x86_64.
- **Done when:** `cargo build --release` cross-compiles for aarch64 and the binary serves a page on your Orange Pi.

### M1 — Content + ingest (week 2)
- [ ] Implement content JSON schema + provenance validation (reject = specific error naming the missing field).
- [ ] `librarian ingest <dir>`: parse → chunks → SQLite (documents, chunks, FTS5), idempotent via file hash in `ingest_log`.
- [ ] Port Paraclea's ingestion code as a starting point; adapt to this schema.
- [ ] Convert 4 real sources: FM 21-76, one Gutenberg book, KJV Bible, and one star-navigation handbook JSON (Half A of the constellations plan, §M8 — same schema, no new architecture).
- **Done when:** `librarian ingest content/` twice in a row produces no duplicates, and a corrupted provenance block is rejected.

### M2 — Search + Reader (week 3)
- [ ] `/api/search` → FTS5 with snippet() highlighting.
- [ ] Reader UI: browse + read + search. Provenance footer.
- **Done when:** on a Pi 4, search over ~5 MB of text returns in < 100 ms. You now have a product without AI.

### M3 — Librarian (week 4)
- [ ] Ollama client with SSE streaming; configurable endpoint; graceful "librarian offline" degradation.
- [ ] Retrieval → prompt assembly → streamed answer + citations UI.
- [ ] Prompt rules per §6; write 20 test questions, verify the "I don't have that" refusal works.
- **Done when:** on the Orange Pi, "How do I purify water?" returns an answer citing FM 21-76 in under ~10 s, and "How do I fix a car transmission?" is refused.

### M4 — Galaxy (week 5–6)
- [ ] `/api/galaxy` graph JSON.
- [ ] Port your HTML galaxy UI; 2D fallback; click → Reader.
- **Done when:** galaxy renders on a mid-range phone browser over Wi-Fi without jank; graph = 1000+ nodes.

### M5 — Maps (week 1 sprint)
- [ ] pmtiles range-serving from the server; MapLibre viewer; region list.
- [ ] Download script (online) for your region: `librarian maps fetch <bbox>`.
- **Done when:** a country map (e.g., ~500 MB pmtiles) browses offline on the SBC.

### M6 — Notes (week 1 sprint)
- [ ] SQLite notes + markdown editor + `[[doc#section]]` links + galaxy nodes for notes.

### M7 — Polish & release (week 2 sprint)
- [ ] `librarian doctor`: disk, RAM, index health, LLM reachability — one command diagnosis.
- [ ] Install script for ARM64; profile auto-detect by free RAM.
- [ ] **Code-review regression suite** (from the 2026-10 review of mazzaroth-harness), as `cargo test` integration tests:
  - [ ] Ingest rejects a document whose provenance came from a code-generated default (fabricated-provenance test).
  - [ ] License validator rejects `cc-by-nc`, `cc-by-nd`, and any string merely *containing* "mit"/"open source" — exact enum match only.
  - [ ] SSE `/api/ask` stream survives a slow (>30 s) mocked LLM without disconnecting (no whole-request timeout).
  - [ ] `num_ctx` equals the active profile's `max_context_tokens()`.
  - [ ] All tests pass from a clean checkout with **no** sibling `galaxy/` directory present (no tainted-corpus dependency, no CWD probing).
- [ ] Docs: README, SCHEMA.md, SOURCING.md; publish with the PD-only demo corpus.
- **v1.0 = M0–M7.**

### M8+ — Stretch backlog (in rough order)
TTS for sections and librarian answers (port Pocket TTS) → **Sky Deck** (see below) → optional embedded embeddings + hybrid rerank for better retrieval → mesh comms port → e-ink/terminal UI → your private (non-redistributable) corpus as an untracked `content/personal/` dir.

#### Constellations: two halves, ship them at different times
**Half A — Star navigation handbook → ingest NOW (fits M1's pipeline, zero new architecture).** Navigation by stars (Polaris north, Southern Cross south, pointer stars), season-finding (heliacal risings, Orion = winter), and cultural star lore from Stellarium's skycultures are ordinary JSON documents in the existing schema: same provenance gate, same reader, same librarian citations. Add these to the M1 conversion list alongside FM 21-76.

**Half B — Sky Deck renderer → M8, after v1.** A live view: date/time + location → rendered night sky with constellation lines. The alt/az math from RA/Dec is trivial (~200 lines of Rust: local sidereal time + one rotation matrix; `astro`/`novas` crates exist if needed). New API: `/api/sky?lat=&lon=&time=`; new UI view sharing the galaxy's canvas conventions.

**Hard boundary:** Sky Deck = stars + constellation lines + links into the handbook. No planets, no moon phases, no satellites, no ephemerides — each of those is a real astronomy project and the start of "offline planetarium" scope creep. A diagram in the handbook teaches the Big Dipper's pointer stars; the Sky Deck just makes it interactive.

---

## 9. Risk register & troubleshooting

| Risk / symptom | Prevention or fix |
|---|---|
| **OOM on SBC** when model loads | Enforce profile RAM caps; check free RAM at startup in `doctor`; Q4 quant only; keep context 2k on `standard`. |
| **Answer quality poor / hallucination** | It's retrieval first: check FTS5 hit quality (run the query yourself), raise top-k, improve chunking at section boundaries, tighten prompt rules. A 3B model can't compensate for bad chunks. |
| **Search slow** | FTS5 external-content table + `optimize()` after ingest; never `LIKE '%…%'`. |
| **UI jank on phone** | 2D fallback; cap galaxy nodes (aggregate sections into doc-level nodes above ~2k); test on real phone, not desktop. |
| **Copyright takedown** | Provenance gate at ingest + only PD/CC content shipped; `personal-use-only` flag for your private library; never commit the tainted Paraclea Bible data. |
| **Fabricated provenance** (found in code review 2026-10) | The gate validates *self-declared* strings — code can lie on the form. Never stamp metadata programmatically; every conversion path (`get_book_as_document`, Gutenberg/FM tools) must read license from a per-source manifest and **reject unknown sources**. Regression test: ingest must fail for any document whose provenance was generated by a default/fallback path. |
| **Medical content outdated/wrong** | Mandatory source-date display; never let the model paraphrase dosages; prefer USG sources with dates; add a "verify against printed card" disclaimer for the medical category. |
| **Ollama dies mid-answer** | Stream with timeout; surface partial answer + "librarian went offline" note; retrieval links remain valid. **Do NOT set a whole-request timeout** — reqwest's `timeout()` covers the entire streamed body, and a 3B on CPU regularly exceeds 15 s total; the stream dies mid-sentence. Use `connect_timeout` or per-chunk read timeouts only. |
| **Context silently truncated** | `num_ctx` must be wired to `HardwareProfile::max_context_tokens()`, never hardcoded. Budget the assembled context (chars ≈ 4× tokens) against it — otherwise Ollama truncates evidence and the librarian answers from partial passages. |
| **LAN-exposed API attacked via browser** | No permissive CORS on a `0.0.0.0`-bound server — any webpage you visit can call the box (delete notes/maps, trigger ingestion). Same-origin UI needs no CORS at all. |
| **CWD-dependent behavior** | All paths flow from `LibrarianConfig`; no relative-path probing of sibling directories and no hardcoded legacy paths (`data/mazzaroth.db`, `galaxy/data/bibles`). Behavior must be identical regardless of launch directory. Add a test that runs the binary from `/` and asserts identical behavior. |
| **Disk full** (maps + library) | `doctor` reports disk; maps fetched only on request; SQLite in WAL mode. |
| **Cross-compile pain** | M0 already builds on aarch64 CI; use `cross` or build on-device; the Orange Pi itself is a fine build machine. |
| **Scope creep** | Any new feature idea goes to M8 backlog, not the milestone. Non-goals list is law. |

---

## 10. Acceptance test (the demo script for yourself)

Run on the Orange Pi, no internet:
1. Power on → browser → Reader opens.
2. Search "water purification" → < 100 ms, FM 21-76 hits.
3. Ask the librarian "How do I purify water?" → cited answer with date.
4. Ask something not in the library → refused.
5. Open galaxy → find the survival cluster → click into a chapter.
6. Open the map → pan around your region.
7. Write a note linking the passage you just read.
8. Unplug nothing, reboot the Pi → all of the above still works.

If that passes, you've beaten the box NOMAD tells people to buy a GPU for.

---

*Draft 1 — expected to be torn apart. Update sections in place as decisions change; do not fork this doc.*