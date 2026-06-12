// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

pub fn check_boot_log(log: &str) -> Result<u64, String> {
    if log.contains("[ZK-ATTEST] FAIL") {
        return Err("boot log contains a failed attestation".into());
    }
    let ok = log.matches("[ZK-ATTEST] ok").count() as u64;
    if ok == 0 {
        return Err("boot log contains no successful attestation".into());
    }
    Ok(ok)
}
