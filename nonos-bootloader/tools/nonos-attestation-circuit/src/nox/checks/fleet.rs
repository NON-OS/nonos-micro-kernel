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

pub fn check_fleet(report: &str, vk_sha256: &[u8; 32]) -> Result<u64, String> {
    let line = report
        .lines()
        .find(|l| l.starts_with("# ") && l.contains(" failed against verifying-key "))
        .ok_or("fleet report has no verdict line")?;
    let rest = line.trim_start_matches("# ");
    let (verified, rest) = rest.split_once(" verified, ").ok_or("malformed verdict line")?;
    let (failed, fpr) =
        rest.split_once(" failed against verifying-key ").ok_or("malformed verdict line")?;
    let verified: u64 = verified.trim().parse().map_err(|_| "verified count not a number")?;
    let failed: u64 = failed.trim().parse().map_err(|_| "failed count not a number")?;
    if failed != 0 {
        return Err(format!("fleet report records {failed} failed capsules"));
    }
    if verified == 0 {
        return Err("fleet report records zero verified capsules".into());
    }
    if fpr.trim() != &hex::encode(vk_sha256)[..16] {
        return Err("fleet report verifying-key fingerprint does not match the key".into());
    }
    Ok(verified)
}
