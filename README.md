# 🌌 Mazzaroth: Celestial Cognitive Memory System

Mazzaroth is a local-first, sovereign cognitive memory engine and 3D celestial visualizer that models human-like long-term memory as an evolving 3D particle galaxy.

## 🚀 Quickstart

### 1. Run the Mazzaroth Daemon & WebGL Visualizer
```bash
cargo run --bin mazzaroth
```
Open **[http://localhost:8080](http://localhost:8080)** in your browser to interact with the 3D WebGL particle galaxy.

### 2. Run the Native Desktop 3D Galaxy (egui / OpenGL)
```bash
cargo run --bin mazzaroth-gui
```

### 3. Model Context Protocol (MCP) Integration
Mazzaroth exposes a standard MCP JSON-RPC 2.0 endpoint at `http://localhost:8080/api/mcp` with tools:
- `mazzaroth_remember(label, content, tier, tags)`
- `mazzaroth_recall(query, limit)`
- `mazzaroth_get_galaxy()`
