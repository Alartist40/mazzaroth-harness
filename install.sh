#!/usr/bin/env bash
set -e

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

echo "Building Mazzaroth release binary..."
cargo build --release --locked --bin mazzaroth

WRAPPER="$BIN_DIR/mazzaroth"
cat <<WRAPPER_EOF > "$WRAPPER"
#!/usr/bin/env bash
# Quick check if Mazzaroth daemon is already listening on default port
if curl -s -m 1 http://127.0.0.1:8080/health >/dev/null 2>&1; then
    echo "🌌 Mazzaroth is already running. Opening browser..."
    if command -v xdg-open >/dev/null 2>&1; then
        xdg-open http://127.0.0.1:8080 >/dev/null 2>&1 &
    fi
    exit 0
fi

cd "$REPO_DIR"
exec ./target/release/mazzaroth "\$@"
WRAPPER_EOF

chmod +x "$WRAPPER"
echo "Installed mazzaroth wrapper to $WRAPPER"
