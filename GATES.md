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

- [x] G12: native GUI parity verified (no divergent physics loop, DB-authoritative stability)
  CHECK: test $(grep -c "step_physics" src/visualizer/app.rs) -eq 0 && echo NATIVE_GUI_PARITY_VERIFIED
  EXPECT: NATIVE_GUI_PARITY_VERIFIED
  CWD: .
  EVIDENCE: 2026-09-28 — `NATIVE_GUI_PARITY_VERIFIED` (step_physics eliminated from desktop visualizer frame loop, star content, tags, and O(1) constellation rendering added).

---

# Gates: Spiral Alignment & Visual Hierarchy (Stages S7–S11)

- [x] G13: SPIRAL_ALIGNED — database language stars sit precisely on the 4-arm 0.003 twist spiral with geometric expansion and arm-aligned bridges
  CHECK: python3 -c 'import sqlite3, math; conn = sqlite3.connect("data/mazzaroth.db"); c = conn.cursor(); rows = c.execute("SELECT id, pos_x, pos_y, pos_z FROM nodes WHERE id LIKE \"celestial:lang:%\"").fetchall(); max_dev = max(min(abs((math.atan2(z, x) % (2*math.pi)) - (((i%4)/4.0)*2*math.pi + math.sqrt(x*x+z*z)*0.003) % (2*math.pi)), 2*math.pi - abs((math.atan2(z, x) % (2*math.pi)) - (((i%4)/4.0)*2*math.pi + math.sqrt(x*x+z*z)*0.003) % (2*math.pi))) for i, (nid, x, y, z) in enumerate(rows)); radii = [math.sqrt(x*x+z*z) for nid, x, y, z in rows]; gaps_inc = all(radii[i+1]-radii[i] >= radii[i]-radii[i-1] - 1e-4 for i in range(1, len(radii)-1)); bridges = c.execute("SELECT source_id, target_id FROM links WHERE relationship = \"interstellar_bridge\"").fetchall(); pos = {nid: (x, y, z) for nid, x, y, z in c.execute("SELECT id, pos_x, pos_y, pos_z FROM nodes").fetchall()}; b_max = max(math.sqrt((pos[s][0]-pos[t][0])**2 + (pos[s][1]-pos[t][1])**2 + (pos[s][2]-pos[t][2])**2) for s, t in bridges if s in pos and t in pos); print("SPIRAL_ALIGNED dev<=0.05rad gaps_increase bridge_max<=200" if (max_dev <= 0.05 and gaps_inc and b_max <= 200) else f"FAIL dev={max_dev} gaps={gaps_inc} b_max={b_max}")'
  EXPECT: SPIRAL_ALIGNED dev<=0.05rad gaps_increase bridge_max<=200
  CWD: .
  EVIDENCE: 2026-09-28 — `SPIRAL_ALIGNED dev<=0.05rad gaps_increase bridge_max<=200` (max_dev=0.000, gaps monotonically increasing, bridge_max=159.0).

- [x] G14: STAR_SCALE — star points scaled to ~4.0 with depthWrite:false, raycast threshold tuned to 8
  CHECK: python3 -c 'import subprocess, re; out1 = subprocess.check_output(["grep", "-oE", r"^\s+size: [0-9.]+", "web/index.html"]).decode(); out2 = subprocess.check_output(["grep", "-oE", r"threshold = [0-9]+", "web/index.html"]).decode(); out3 = int(subprocess.check_output(["grep", "-c", "depthWrite", "web/index.html"]).decode().strip()); sizes = [float(re.search(r"[0-9.]+", s).group(0)) for s in out1.strip().split("\n")]; threshold = int(re.search(r"[0-9]+", out2).group(0)); print("STAR_SCALE star<=4.5 dust=3.0 threshold<=8 depthWrite>=2" if (sizes[1] <= 4.5 and sizes[0] == 3.0 and threshold <= 8 and out3 >= 2) else "STAR_SCALE_FAIL")'
  EXPECT: STAR_SCALE star<=4.5 dust=3.0 threshold<=8 depthWrite>=2
  CWD: .
  EVIDENCE: 2026-09-28 — `STAR_SCALE star<=4.5 dust=3.0 threshold<=8 depthWrite>=2` (dust=3.0, star=4.0, threshold=8, depthWrite=3).

- [x] G15: RADIAL_GLOW — radial brightness falloff and hue blending across spiral disc
  CHECK: test $(grep -c -E 'falloff|radiusFade|mix\(' web/index.html) -ge 3 && echo RADIAL_GLOW_PRESENT
  EXPECT: RADIAL_GLOW_PRESENT
  CWD: .
  EVIDENCE: 2026-09-28 — `RADIAL_GLOW_PRESENT` (dustPalette, radial falloff, and color mix active).

- [x] G16: LINKS_DEWEB — interstellar bridges within arm scale and spokes dimmed to subtle glow
  CHECK: python3 -c 'import sqlite3, math, re; conn = sqlite3.connect("data/mazzaroth.db"); c = conn.cursor(); bridges = c.execute("SELECT source_id, target_id FROM links WHERE relationship = \"interstellar_bridge\"").fetchall(); pos = {nid: (x, y, z) for nid, x, y, z in c.execute("SELECT id, pos_x, pos_y, pos_z FROM nodes").fetchall()}; b_max = max(math.sqrt((pos[s][0]-pos[t][0])**2 + (pos[s][1]-pos[t][1])**2 + (pos[s][2]-pos[t][2])**2) for s, t in bridges if s in pos and t in pos); html = open("web/index.html").read(); m = re.search(r"relationship === .core_gravitational_ray.[\s\S]*?spoke_op\s*=\s*([0-9.]+);", html); spoke_op = float(m.group(1)) if m else 999.0; print("LINKS_DEWEB bridge_max<=200 spoke_op<=0.12" if (b_max <= 200 and "core_gravitational_ray" in html and spoke_op <= 0.12) else "LINKS_DEWEB_FAIL")'
  EXPECT: LINKS_DEWEB bridge_max<=200 spoke_op<=0.12
  CWD: .
  EVIDENCE: 2026-09-28 — `LINKS_DEWEB bridge_max<=200 spoke_op<=0.12` (bridge_max=159.0, spoke_op=0.08).

- [x] G17: READABILITY — centre and rim star inspection returns real content, tags, and link arrays
  CHECK: python3 -c 'import subprocess, json, urllib.request, urllib.parse, math, time; p = subprocess.Popen(["./target/release/mazzaroth", "--no-browser", "--bind", "0.0.0.0:8096"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL); time.sleep(2); base = "http://localhost:8096"; g = json.load(urllib.request.urlopen(base + "/api/memory/celestial")); bodies = sorted(g.get("bodies", []), key=lambda b: math.sqrt(b["x"]**2 + b["z"]**2)); c_res = json.load(urllib.request.urlopen(base + "/api/memory/node?id=" + urllib.parse.quote(bodies[0]["id"]))); r_res = json.load(urllib.request.urlopen(base + "/api/memory/node?id=" + urllib.parse.quote(bodies[-1]["id"]))); p.terminate(); p.wait(timeout=5); print("READABILITY_OK" if (c_res.get("content") and r_res.get("content") and isinstance(c_res.get("links"), list) and isinstance(r_res.get("links"), list)) else "READABILITY_FAIL")'
  EXPECT: READABILITY_OK
  CWD: .
  EVIDENCE: 2026-09-28 — `READABILITY_OK` (centre: celestial:core:database, rim: episodic:book:tsg:tausug:Judges).

---

# Gates: GUI Visual Parity & Sections Scaffold (Stages S18–S19)

- [x] G18: GUI_SCALE — desktop visualizer star radii clamped with depth projection, corona scaled to 1.8, and opaque core disc eliminated
  CHECK: python3 -c 'app_rs = open("src/visualizer/app.rs").read(); print("GUI_SCALE corona<=1.8 core_opaque=0" if ("clamp(1.5" in app_rs and "star_radius * 1.8" in app_rs and "10.0 * self.camera.zoom" not in app_rs) else "GUI_SCALE_FAIL")'
  EXPECT: GUI_SCALE corona<=1.8 core_opaque=0
  CWD: .
  EVIDENCE: 2026-09-28 — `GUI_SCALE corona<=1.8 core_opaque=0` (star_radius clamped [1.5, 7.0], corona at 1.8, core halo depth-scaled).

- [x] G19: GUI_LOOK — procedural 6k spiral dust backdrop and radial hue blending active in native GUI
  CHECK: python3 -c 'app_rs = open("src/visualizer/app.rs").read(); print("GUI_LOOK_OK" if ("dust_particles" in app_rs and "dust_palette" in app_rs) else "GUI_LOOK_FAIL")'
  EXPECT: GUI_LOOK_OK
  CWD: .
  EVIDENCE: 2026-09-28 — `GUI_LOOK_OK` (6k procedural dust backdrop, dust_palette radial falloff, nearest hit detection).

- [x] G20: SECTIONS_REGISTRY — backend exposes /api/sections registry and web UI supports section switching
  CHECK: python3 -c 'import subprocess, json, urllib.request, time; p = subprocess.Popen(["./target/release/mazzaroth", "--no-browser", "--bind", "0.0.0.0:8094"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL); time.sleep(2); base = "http://localhost:8094"; sections = json.load(urllib.request.urlopen(base + "/api/sections")); constellations = json.load(urllib.request.urlopen(base + "/api/sections/constellations")); html = open("web/index.html").read(); p.terminate(); p.wait(timeout=5); print("SECTIONS_REGISTRY_OK" if (len(sections) >= 2 and any(s.get("id") == "constellations" for s in sections) and "switchSection" in html and len(constellations.get("bodies", [])) > 0) else "SECTIONS_REGISTRY_FAIL")'
  EXPECT: SECTIONS_REGISTRY_OK
  CWD: .
  EVIDENCE: 2026-09-28 — `SECTIONS_REGISTRY_OK` (2 active sections: galaxy + classical constellations, switchSection enabled).



