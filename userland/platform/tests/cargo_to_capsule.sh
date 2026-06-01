#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# End-to-end Cargo-to-Capsule proof. Signs a package through the real
# hybrid signer (Ed25519 + ML-DSA) under the baked trust anchor, then
# exercises the nonos CLI install path: a valid package installs, a
# tampered payload is refused, a tampered manifest is refused, an
# unsigned manifest is refused, and remove cleans up.

set -u
REPO="$(cd "$(dirname "$0")/../../.." && pwd)"
SIGN="${REPO}/nonos-sign/target/release/capsule-sign"
NONOS="${REPO}/userland/platform/nonos_capsule/target/release/nonos"
TA_ED="${REPO}/.keys/nonos_trust_anchor_ed25519.seed"
TA_ML="${REPO}/.keys/nonos_trust_anchor_mldsa65.seed"
POLICY="${REPO}/nonos-data/trust/policy/nonos_trust_anchor.policy.bin"
EPOCH=1
VFROM=1767225600000
VUNTIL=1893456000000

for f in "$SIGN" "$NONOS" "$TA_ED" "$TA_ML" "$POLICY"; do
    [ -e "$f" ] || { echo "missing prerequisite: $f"; exit 2; }
done

W="$(mktemp -d)"
trap 'rm -rf "$W"' EXIT
export NONOS_STORE="${W}/store"
export NONOS_SIGNER="$SIGN"
fail=0
ok()      { if [ "$1" -eq 0 ]; then echo "PASS: $2"; else echo "FAIL: $2"; fail=1; fi; }
refused() { if [ "$1" -ne 0 ]; then echo "PASS (refused): $2"; else echo "FAIL (accepted!): $2"; fail=1; fi; }

"$SIGN" keygen --alg ed25519 --out "${W}/pub_ed" >/dev/null
"$SIGN" keygen --alg mldsa65 --out "${W}/pub_ml" >/dev/null
NID="$("$SIGN" derive-id --handle testapp --domain systems.nonos --recovery none)"
"$SIGN" sign-id-cert --serial 1 --nonos-id "$NID" --ns-glob "systems.nonos.*" \
    --caps-ceiling 0xffff --epoch "$EPOCH" --valid-from-ms "$VFROM" --valid-until-ms "$VUNTIL" \
    --pub-key ed25519="${W}/pub_ed.pub" --pub-key mldsa65="${W}/pub_ml.pub" \
    --ta-seed ed25519="$TA_ED" --ta-seed mldsa65="$TA_ML" \
    --metadata test --out "${W}/publisher.cert" >/dev/null || { echo "cert failed"; exit 1; }

printf 'NONOS-CAPSULE-PAYLOAD-deterministic-bytes' > "${W}/payload.elf"
"$SIGN" sign-manifest --cert "${W}/publisher.cert" --namespace systems.nonos.testapp \
    --version 1.0.0 --target x86_64-nonos-user --elf "${W}/payload.elf" \
    --required-caps 0x19 --optional-caps 0x0 --endpoint service:5000:testapp.svc \
    --pub-seed ed25519="${W}/pub_ed.seed" --pub-seed mldsa65="${W}/pub_ml.seed" \
    --out "${W}/manifest.nmf" >/dev/null || { echo "manifest failed"; exit 1; }

pkg="${W}/pkg"; mkdir -p "$pkg"
cp "${W}/payload.elf" "${pkg}/payload.elf"
cp "${W}/manifest.nmf" "${pkg}/manifest.nmf"
printf 'testapp\n1.0.0\nx86_64-nonos-user\nsystems.nonos.testapp\n' > "${pkg}/package.meta"

"$NONOS" install "$pkg" --cert "${W}/publisher.cert" --policy "$POLICY"; ok $? "install valid signed package"
"$NONOS" inspect testapp >/dev/null; ok $? "inspect installed capsule"

tp="${W}/tp"; cp -r "$pkg" "$tp"; printf 'X' >> "${tp}/payload.elf"
"$NONOS" install "$tp" --cert "${W}/publisher.cert" --policy "$POLICY" 2>/dev/null; refused $? "tampered payload"

tm="${W}/tm"; cp -r "$pkg" "$tm"
sz=$(wc -c < "${tm}/manifest.nmf")
printf '\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff' \
    | dd of="${tm}/manifest.nmf" bs=1 seek=$((sz - 16)) count=16 conv=notrunc 2>/dev/null
"$NONOS" install "$tm" --cert "${W}/publisher.cert" --policy "$POLICY" 2>/dev/null; refused $? "tampered manifest"

us="${W}/us"; cp -r "$pkg" "$us"; head -c 256 /dev/urandom > "${us}/manifest.nmf"
"$NONOS" install "$us" --cert "${W}/publisher.cert" --policy "$POLICY" 2>/dev/null; refused $? "unsigned/garbage manifest"

"$NONOS" remove testapp; ok $? "remove installed capsule"
"$NONOS" inspect testapp 2>/dev/null; refused $? "inspect after remove fails"

[ "$fail" -eq 0 ] && echo "ALL PASS" || { echo "SOME FAILED"; exit 1; }
