#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify it under
# the terms of the GNU Affero General Public License as published by the Free
# Software Foundation, either version 3 of the License, or (at your option) any
# later version. See <https://www.gnu.org/licenses/>.
#
# The proof-corpus root: a reproducible 32-byte commitment over the machine
# checked Lean corpus. It is what binds "these properties were proven" to the
# attested kernel image. The root is a hash over, in canonical order:
#
#   - every Nonos Lean source (path + sha256), so a changed proof changes the
#     root;
#   - the pinned lean-toolchain, so the checker version is part of the claim;
#   - the axiom summary emitted by AxiomProfile, so the root only forms when
#     every load-bearing theorem depends on Lean's standard axioms alone.
#
# The build must pass and the axiom profile must be clean (no sorryAx, no axiom
# outside the standard three) or this script exits non-zero and prints nothing.
# Run from verification/lean. Feed the printed root to nonos-stark-enroll as the
# kernel attestation's verification public input.

set -euo pipefail
cd "$(dirname "$0")"

case "$(uname -s)" in
  Darwin) SHA256="shasum -a 256" ;;
  *)      SHA256="sha256sum" ;;
esac
sha() { $SHA256 "$1" | cut -d' ' -f1; }
sha_stdin() { $SHA256 | cut -d' ' -f1; }

# 1. The proofs must actually check.
lake build >/dev/null 2>&1 || { echo "proof-corpus-root: lake build failed" >&2; exit 1; }

# 2. The axiom profile must be clean: standard axioms only, never sorryAx.
axioms="$(lake env lean AxiomProfile.lean 2>/dev/null)"
if printf '%s' "$axioms" | grep -qiE 'sorryAx|sorry'; then
  echo "proof-corpus-root: AxiomProfile references sorry; refusing to commit" >&2
  exit 1
fi
# Every axiom line must name only the standard three. Any other axiom is a hole.
bad="$(printf '%s\n' "$axioms" \
  | grep -oE "'[^']+'" \
  | grep -vE "Nonos\.|propext|Classical\.choice|Quot\.sound" || true)"
if [ -n "$bad" ]; then
  echo "proof-corpus-root: non-standard axiom in corpus:" >&2
  printf '%s\n' "$bad" >&2
  exit 1
fi

# 3. Canonical manifest: sorted source hashes, the toolchain, the axiom digest.
manifest="$(mktemp)"
trap 'rm -f "$manifest"' EXIT
{
  echo "toolchain $(cat lean-toolchain)"
  # Deterministic order: sort by path with the C locale so the root is
  # host-independent.
  while IFS= read -r f; do
    printf '%s %s\n' "$f" "$(sha "$f")"
  done < <(find Nonos -name '*.lean' | LC_ALL=C sort)
  # The axiom profile output, hashed, folds the proven-clean fact into the root.
  printf 'axioms %s\n' "$(printf '%s' "$axioms" | sha_stdin)"
} > "$manifest"

root="$(sha "$manifest")"
count="$(grep -rhc '^theorem ' Nonos/*.lean Nonos/Stark/*.lean | awk '{s+=$1} END{print s}')"
modules="$(find Nonos -name '*.lean' | wc -l | tr -d ' ')"

echo "proof-corpus-root: $root"
echo "  modules:  $modules"
echo "  theorems: $count"
echo "  toolchain: $(cat lean-toolchain)"
