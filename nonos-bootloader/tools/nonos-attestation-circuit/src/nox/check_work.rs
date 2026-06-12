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

use std::path::Path;

use super::checks::{check_audit, check_boot_log, check_capsule, check_fleet};
use super::kind::WorkKind;

pub fn check_work(
    kind: WorkKind,
    artifact_path: &Path,
    artifact: &[u8],
    vk_path: &Path,
    vk_sha256: &[u8; 32],
) -> Result<u64, String> {
    match kind {
        WorkKind::FleetVerification => {
            let text = std::str::from_utf8(artifact).map_err(|_| "fleet report is not UTF-8")?;
            check_fleet(text, vk_sha256)
        }
        WorkKind::RuntimeBoot | WorkKind::HardwareBoot => {
            let text = std::str::from_utf8(artifact).map_err(|_| "boot log is not UTF-8")?;
            check_boot_log(text)
        }
        WorkKind::CircuitAudit | WorkKind::CapsuleAudit => {
            check_audit(artifact)?;
            Ok(0)
        }
        WorkKind::CapsuleBuild => {
            check_capsule(vk_path, artifact_path)?;
            Ok(1)
        }
    }
}
