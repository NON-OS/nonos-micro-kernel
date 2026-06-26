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
import json, sys
required = ("vendor", "model", "cpu", "firmware", "gpu", "display_path", "storage", "input", "iommu", "serial_capture")
if len(sys.argv) != 2:
    raise SystemExit("usage: validate_machine_metadata.py <machine.json>")
with open(sys.argv[1], encoding="utf-8") as src:
    data = json.load(src)
missing = [k for k in required if not str(data.get(k, "")).strip()]
if data.get("schema") != "nonos.hardware.machine.v1":
    missing.insert(0, "schema=nonos.hardware.machine.v1")
if missing:
    print("machine metadata gap: " + ", ".join(missing))
    raise SystemExit(2)
print("machine metadata ok")
