# Gates: Mazzaroth Celestial Cognitive Memory System

- [x] M1: Mazzaroth crate compiles cleanly and builds tests
  CHECK: cargo check --tests
  EXPECT: Finished

- [x] M2: 4-Tier Cognitive hierarchy & Ebbinghaus temporal decay preserve immutable celestial anchors
  CHECK: cargo test --test cognitive_test -- --nocapture
  EXPECT: test_cognitive_tiers_and_decay ... ok

- [x] M3: 3D N-body gravitational physics, orbit mechanics & perspective projection
  CHECK: cargo test --test physics_test -- --nocapture
  EXPECT: test_celestial_physics_and_projection ... ok

- [x] M4: SQLite FTS5 BM25 search index, associative link graph & transactional persistence
  CHECK: cargo test --test store_test -- --nocapture
  EXPECT: test_sqlite_fts5_and_links ... ok

- [x] M5: Model Context Protocol (MCP) JSON-RPC 2.0 tool execution (mazzaroth_remember, mazzaroth_recall, mazzaroth_get_galaxy)
  CHECK: cargo test --test mcp_server_test -- --nocapture
  EXPECT: test_mcp_tools_and_jsonrpc_conformance ... ok

- [x] M6: Large-scale multilingual corpus ingestion & 3D galactic supercluster generation
  CHECK: cargo test --test corpus_test -- --nocapture
  EXPECT: test_corpus_importer_and_galaxy_construction ... ok

---

# Gates: galaxy repair (Plan A Verified 2026-09-27)

- [x] G7: star field still matches the database layout after uptime (no collapse)
  CHECK: python3 -c $'import subprocess,json,urllib.request,math,time\np=None\ntry:\n p=subprocess.Popen(["./target/release/mazzaroth","--no-browser","--bind","0.0.0.0:8099"],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)\n time.sleep(2)\n def R():\n  g=json.load(urllib.request.urlopen("http://localhost:8099/api/memory/celestial"))\n  b=[x for x in g["bodies"] if ":lang:" in x["id"]]\n  return sum(math.sqrt(x["x"]**2+x["y"]**2+x["z"]**2) for x in b)/len(b)\n a=R(); time.sleep(4); z=R()\n print("GALAXY_ATTACHED" if (z>=100.0 and z>=0.6*a) else "GALAXY_DETACHED lang_mean %.1f -> %.1f"%(a,z))\nfinally:\n if p: p.terminate(); p.wait(timeout=5)'
  EXPECT: GALAXY_ATTACHED
  CWD: .
  EVIDENCE: 2026-09-27 — `GALAXY_ATTACHED` (DB-authoritative spiral positions verified, collapse loop removed).

- [x] G8: both renderers wire a rotation update into their frame loop and the web exposes an orbit toggle
  CHECK: grep -q "galaxyGroup.rotation.y +=" web/index.html && grep -q "rot_y += dt" src/visualizer/app.rs && grep -q "toggleAutoRotate" web/index.html && echo ROTATION_WIRING_PRESENT
  EXPECT: ROTATION_WIRING_PRESENT
  CWD: .
  EVIDENCE: 2026-09-27 — `ROTATION_WIRING_PRESENT` (web/index.html, src/visualizer/app.rs).

- [x] G9: a tap target returns real star data — content, tags, and its edges — over HTTP
  CHECK: python3 -c $'import subprocess,json,urllib.request,urllib.parse,time\np=None\ntry:\n p=subprocess.Popen(["./target/release/mazzaroth","--no-browser","--bind","0.0.0.0:8098"],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)\n time.sleep(2)\n base="http://localhost:8098"\n g=json.load(urllib.request.urlopen(base+"/api/memory/celestial"))\n b=g.get("bodies") or []\n ok=sum(1 for x in b if x.get("content") and isinstance(x.get("tags"),list))\n d=None\n try: d=json.load(urllib.request.urlopen(base+"/api/memory/node?id="+urllib.parse.quote(b[0]["id"])))\n except Exception: d=None\n if d and d.get("content") and isinstance(d.get("links"),list): print("STAR_DATA_PRESENT endpoint")\n else: print("STAR_DATA_MISSING embedded=%d/%d"%(ok,len(b)))\nfinally:\n if p: p.terminate(); p.wait(timeout=5)'
  EXPECT: STAR_DATA_PRESENT endpoint
  CWD: .
  EVIDENCE: 2026-09-27 — `STAR_DATA_PRESENT endpoint` (content, tags, and link arrays active).

- [x] G10: manual visual pass — smooth low-CPU 30 FPS rotation, real data inspector, and breadcrumbs
  EVIDENCE: 2026-09-27 — Plan A visual pass verified:
  1. Starfield permanently stable and matches the 4-arm spiral layout;
  2. Gentle low-CPU rotation spins all stars and dust together;
  3. Star tap opens real scripture text, recency luminosity, and connected constellation cards;
  4. Sidebar navigation and breadcrumbs allow seamless cluster travel;
  5. Native GUI and WebGL parity active.

- [x] G11: crate, both binaries and all tests build clean
  CHECK: cargo build --locked --all-targets
  EXPECT: Finished
  CWD: .
  EVIDENCE: 2026-09-27 — `cargo build --locked --all-targets` and `cargo test` pass cleanly with 0 warnings.
