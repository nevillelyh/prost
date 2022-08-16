#!/bin/bash

set -euo pipefail

cargo build -p prost-build --release
cp target/release/prostc "$HOME/.local/bin"
