#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# The security suite harness: build the tools, run the red-team battery, check
# the JSON report holds, and fuzz the trailer parser. Same sequence CI runs.

set -euo pipefail

crate="$(cd "$(dirname "$0")/.." && pwd)/nonos-secops"
run() { cargo run --quiet --release --manifest-path "$crate/Cargo.toml" --bin nonos-attack -- "$@"; }
iters="${1:-20000}"

echo "== red-team battery"
run battery

echo "== battery JSON report holds"
run --json | grep -q '"held":true'

echo "== parser fuzz ($iters iterations)"
run fuzz "$iters"

echo "== security suite passed"
