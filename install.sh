#!/usr/bin/env bash
set -e

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

echo "Building Mazzaroth release binary..."
cd "$REPO_DIR"
cargo build --release

WRAPPER="$BIN_DIR/mazzaroth"
rm -f "$BIN_DIR/mazzaroth" "$BIN_DIR/librarian"
cat <<WRAPPER_EOF > "$WRAPPER"
#!/usr/bin/env bash
REPO_DIR="$REPO_DIR"

cd "\$REPO_DIR"
if [ \$# -eq 0 ]; then
    # Kill any stale daemon instances on port 8080 to ensure fresh assets are served
    killall -9 mazzaroth 2>/dev/null || true
    echo "✦ Starting Mazzaroth sovereign knowledge daemon on http://127.0.0.1:8080..."
    (sleep 1; if command -v xdg-open >/dev/null 2>&1; then xdg-open http://127.0.0.1:8080 >/dev/null 2>&1; fi) &
    exec ./target/release/mazzaroth serve --bind 127.0.0.1:8080
else
    exec ./target/release/mazzaroth "\$@"
fi
WRAPPER_EOF

chmod +x "$WRAPPER"
ln -sf "$WRAPPER" "$BIN_DIR/librarian"

echo "✓ Installed mazzaroth wrapper to $WRAPPER"
echo "✓ Symlinked librarian to $BIN_DIR/librarian"
