#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

set -euo pipefail
if [ "$#" -lt 1 ]; then
    printf 'usage: SSHPASS=<password> %s user@host [user@host...]\n' "$0" >&2
    exit 2
fi

PASS_FROM_ENV=0
[ -n "${SSHPASS:-}" ] && PASS_FROM_ENV=1

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SRC="$ROOT/nonos-bootloader/tools/nonos-attestation-circuit"
CER="$ROOT/target/external-zk-ceremony"
KEYS="$SRC/generated_keys"
RUN="nonos-zk-$(date +%Y%m%d%H%M%S)"
SSH="ssh -o StrictHostKeyChecking=accept-new -o UserKnownHostsFile=$CER/known_hosts"
RSYNC="rsync -az --delete --exclude target --exclude generated_keys"

mkdir -p "$CER"
make -C "$ROOT" nonos-mk-zk-tools
TOOL="$SRC/target/$(rustc -vV | sed -n 's/^host: //p')/release/run_ceremony"
[ -x "$TOOL" ] || { printf 'run_ceremony binary missing\n' >&2; exit 1; }
rm -rf "$CER"
mkdir -p "$CER"
"$TOOL" init --circuit attestation --output "$CER/params_0.bin"

round=0
records=
for host in "$@"; do
    if [ "$PASS_FROM_ENV" -eq 0 ]; then printf 'ssh password for %s: ' "$host" >&2; IFS= read -rs SSHPASS; printf '\n' >&2; export SSHPASS; fi
    port=22; target="$host"
    case "$host" in *:*) port="${host##*:}"; target="${host%:*}";; esac
    sshcmd="$SSH -p $port"
    in="$CER/params_${round}.bin"
    round=$((round + 1))
    out="params_${round}.bin"
    remote="$RUN-round$round"
    sshpass -e $sshcmd "$target" "rm -rf /tmp/nonos-zk-* '$remote' && mkdir -p '$remote/src' '$remote/work'"
    sshpass -e $RSYNC -e "$sshcmd" "$SRC/" "$target:$remote/src/"
    sshpass -e $RSYNC -e "$sshcmd" "$in" "$target:$remote/work/params_in.bin"
    sshpass -e $sshcmd "$target" "if [ -f \"\$HOME/.cargo/env\" ]; then . \"\$HOME/.cargo/env\"; fi; if ! command -v cargo >/dev/null 2>&1; then apt-get update && apt-get install -y curl build-essential pkg-config ca-certificates || true; curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal; fi"
    sshpass -e $sshcmd "$target" ". \"\$HOME/.cargo/env\" && cd '$remote/src' && cargo build --release --bin run_ceremony"
    sshpass -e $sshcmd "$target" "cd '$remote' && src/target/release/run_ceremony contribute --input work/params_in.bin --output work/$out --name '$host' --location external-linux --entropy system"
    sshpass -e $sshcmd "$target" "cd '$remote/work' && sha256sum '$out' '$out.contribution.json' > round$round.sha256"
    sshpass -e $RSYNC -e "$sshcmd" "$target:$remote/work/$out" "$CER/$out"
    sshpass -e $RSYNC -e "$sshcmd" "$target:$remote/work/$out.contribution.json" "$CER/$out.contribution.json"
    sshpass -e $RSYNC -e "$sshcmd" "$target:$remote/work/round$round.sha256" "$CER/round$round.sha256"
    records="$records $CER/$out.contribution.json"
    [ "$PASS_FROM_ENV" -eq 0 ] && unset SSHPASS
done

"$TOOL" assemble --meta "$CER/params_0.bin.meta.json" --output "$CER/transcript.json" $records
"$TOOL" finalize --input "$CER/params_${round}.bin" --output "$KEYS" --transcript "$CER/transcript.json"
"$TOOL" verify --transcript "$KEYS/ceremony_transcript.json"
cp "$KEYS/attestation_verifying_key.bin" "$KEYS/vk_attestation_program.bin"
cp "$KEYS/attestation_verifying_key.bin" "$KEYS/vk_boot_authority.bin"
cp "$KEYS/attestation_verifying_key.bin" "$KEYS/vk_update_authority.bin"
cp "$KEYS/attestation_verifying_key.bin" "$KEYS/vk_recovery_key.bin"
make -C "$ROOT" nonos-mk-all-capsules-attested nonos-mk-zk-verify-live nonos-mk-zk-report
