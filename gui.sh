#!/usr/bin/env bash
cd "$(dirname "$0")"
DISPLAY="${DISPLAY:-:1}" cargo run --bin mazzaroth-gui
