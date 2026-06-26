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
import hashlib, json, os, shutil, subprocess, sys, time
out = os.environ.get("NONOS_HW_OUT") or "target/hardware-dossier/manual"
serial = os.environ.get("NONOS_HW_SERIAL_LOG") or ""
machine = os.environ.get("NONOS_HW_MACHINE_JSON") or ""
boot_media = os.environ.get("NONOS_HW_BOOT_MEDIA") or ""
extras = [x for x in os.environ.get("NONOS_HW_ARTIFACTS", "").split(":") if x]
required = ("vendor", "model", "cpu", "firmware", "gpu", "display_path", "storage", "input", "iommu", "serial_capture")
os.makedirs(out, exist_ok=True)
report = {"schema": "nonos.hardware.dossier.v1", "created_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "status": "gap", "artifacts": []}
if not serial or not os.path.exists(serial) or os.path.getsize(serial) == 0:
    report["reason"] = "NONOS_HW_SERIAL_LOG missing or empty"
    with open(os.path.join(out, "dossier.json"), "w", encoding="utf-8") as dst:
        json.dump(report, dst, indent=2, sort_keys=True)
        dst.write("\n")
    raise SystemExit(2)
serial_dst = os.path.join(out, "serial.log"); shutil.copy2(serial, serial_dst)
boot_json = os.path.join(out, "boot-log.json"); rc = subprocess.call([sys.executable, "nonos-ci/bench_boot_log.py", serial_dst, boot_json])
machine_ok = False
boot_media_ok = boot_media and os.path.exists(boot_media) and os.path.getsize(boot_media) > 0
if machine and os.path.exists(machine):
    shutil.copy2(machine, os.path.join(out, "machine.json"))
    with open(machine, encoding="utf-8") as src: report["machine"] = json.load(src)
    report["machine_missing"] = [k for k in required if not str(report["machine"].get(k, "")).strip()]
    machine_ok = report["machine"].get("schema") == "nonos.hardware.machine.v1" and not report["machine_missing"]
if boot_media_ok: shutil.copy2(boot_media, os.path.join(out, "boot-media.bin"))
for item in extras:
    if os.path.exists(item): shutil.copy2(item, os.path.join(out, os.path.basename(item)))
if os.path.exists(boot_json):
    with open(boot_json, encoding="utf-8") as src:
        boot = json.load(src)
    report["boot_status"] = boot.get("status", "gap")
    report["markers"] = boot.get("markers", {})
    report["phase_ms"] = boot.get("phase_ms", {})
    report["latency_status"] = boot.get("latency_status", "gap")
    if boot.get("status") == "pass" and machine_ok and boot_media_ok and not report["markers"].get("zk_attest_fail") and report["phase_ms"]:
        report["status"] = "pass"
    elif boot.get("status") == "pass":
        report["reason"] = "machine metadata incomplete, boot media missing, attestation failed, or phases missing"
    else:
        report["reason"] = boot.get("reason", "boot evidence did not pass")
else:
    report["reason"] = "boot parser did not produce boot-log.json"
for root, _, files in os.walk(out):
    for name in sorted(files):
        path = os.path.join(root, name)
        rel = os.path.relpath(path, out)
        if rel in ("dossier.json", "manifest.json"): continue
        with open(path, "rb") as src:
            digest = hashlib.sha256(src.read()).hexdigest()
        report["artifacts"].append({"path": rel, "sha256": digest, "bytes": os.path.getsize(path)})
with open(os.path.join(out, "dossier.json"), "w", encoding="utf-8") as dst:
    json.dump(report, dst, indent=2, sort_keys=True)
    dst.write("\n")
with open(os.path.join(out, "manifest.json"), "w", encoding="utf-8") as dst:
    json.dump({"schema": "nonos.hardware.manifest.v1", "artifacts": report["artifacts"]}, dst, indent=2, sort_keys=True)
    dst.write("\n")
print(os.path.join(out, "dossier.json"))
raise SystemExit(0 if report["status"] == "pass" and rc == 0 else 2)
