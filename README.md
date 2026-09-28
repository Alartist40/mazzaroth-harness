# Mazzaroth: Celestial Cognitive Memory System

Mazzaroth is a local-first, sovereign cognitive memory engine and 3D celestial visualizer that models long-term semantic and episodic memory as an evolving 3D particle spiral galaxy.

## Quickstart

### 1. Install Single-Command Launcher
```bash
./install.sh
```
This builds the release binary and registers `mazzaroth` in `~/.local/bin`.

### 2. Launch
```bash
mazzaroth
```
Opens **http://localhost:8080** in your browser to interact with the 3D particle spiral galaxy, hierarchical scripture navigator, and classical constellation catalogs.

If the daemon is already running, invoking `mazzaroth` automatically focuses your browser window.

### 3. Development Run
```bash
./run.sh
```

### 4. Model Context Protocol (MCP) Integration
Mazzaroth exposes a standard MCP JSON-RPC 2.0 endpoint at `http://127.0.0.1:8080/api/mcp` with tools:
- `mazzaroth_remember(label, content, tier, tags)`
- `mazzaroth_recall(query, limit)`
- `mazzaroth_get_galaxy()`

## Modular Sections Architecture
- `galaxy/`: Multilingual scriptural corpus, 4-arm 0.003-twist logarithmic spiral physics, and scripture reader API.
- `constellation/`: 32 classical northern/southern asterisms and complete 12 zodiac sign catalog (`constellations.json`).
- `web/`: Clean frontend shell (`index.html`, `css/base.css`, `js/main.js`) with 4-step drill-down navigator and reading pane.
