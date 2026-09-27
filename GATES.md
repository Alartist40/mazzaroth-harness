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
