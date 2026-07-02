#!/usr/bin/env python3
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
import glob, json, os, re, sys, time
out = sys.argv[1] if len(sys.argv) > 1 else "target/hardware-support-evidence.json"
src_text = []
for path in sorted(glob.glob("src/**/*.rs", recursive=True)):
    with open(path, encoding="utf-8", errors="ignore") as fh:
        src_text.append((path, fh.read()))
capsules = []
failures = []
for mk in sorted(glob.glob("userland/capsule_driver_*/Capsule.mk")):
    with open(mk, encoding="utf-8") as fh:
        text = fh.read()
    vars = dict(re.findall(r"^\s*(CAPSULE_[A-Z0-9_]+)\s*:=\s*(.*?)\s*$", text, re.M))
    slug = vars.get("CAPSULE_SLUG", "")
    name = vars.get("CAPSULE_BIN_NAME", "")
    cdir = vars.get("CAPSULE_DIR", "")
    service = vars.get("CAPSULE_SERVICE_ENDPOINT", "").split(":")[-1]
    caps = vars.get("CAPSULE_REQUIRED_CAPS", "")
    readme = os.path.join(os.path.dirname(mk), "README.md")
    cert, manifest, trailer = (f"nonos-data/trust/capsules/{name}.nonos_id_cert.bin", f"nonos-data/trust/capsules/{name}.manifest.bin", f"nonos-data/trust/capsules/{name}.zk_trailer.bin")
    hay = "\n".join(body for _, body in src_text if cdir in body or name in body or service in body)
    missing = []
    for key in ("CAPSULE_SLUG", "CAPSULE_BIN_NAME", "CAPSULE_DIR", "CAPSULE_SERVICE_ENDPOINT", "CAPSULE_REPLY_ENDPOINT", "CAPSULE_REQUIRED_CAPS"):
        if not vars.get(key):
            missing.append(key)
    for path in (cert, manifest, trailer):
        if not os.path.getsize(path) if os.path.exists(path) else True:
            missing.append(path)
    if not os.path.getsize(readme) if os.path.exists(readme) else True:
        missing.append(readme)
    if f"{cdir}/target/" not in hay:
        missing.append("kernel embed userland target")
    if f"trust/capsules/{name}.nonos_id_cert.bin" not in hay:
        missing.append("kernel embed identity cert")
    if f"trust/capsules/{name}.manifest.bin" not in hay:
        missing.append("kernel embed manifest")
    if f"trust/capsules/{name}.zk_trailer.bin" not in hay:
        missing.append("kernel embed zk trailer")
    if service not in hay or "spawn_verified" not in hay:
        missing.append("kernel spawn_verified service wiring")
    status = "fail" if missing else "pass"
    if missing:
        failures.append({"capsule": slug or mk, "missing": missing})
    capsules.append({"slug": slug, "bin": name, "service": service, "caps": caps, "status": status})
report = {"schema": "nonos.hardware.support.source.v1", "created_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "status": "fail" if failures else "pass", "capsule_count": len(capsules), "capsules": capsules, "failures": failures}
out_dir = os.path.dirname(out)
if out_dir:
    os.makedirs(out_dir, exist_ok=True)
with open(out, "w", encoding="utf-8") as fh:
    json.dump(report, fh, indent=2, sort_keys=True)
    fh.write("\n")
print(out)
raise SystemExit(0 if not failures else 2)
