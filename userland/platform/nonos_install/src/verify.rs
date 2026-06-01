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
use std::process::Command;

pub fn verify(signer_bin: &str, manifest: &Path, cert: &Path, policy: &Path) -> Result<(), String> {
    let status = Command::new(signer_bin)
        .arg("verify-manifest")
        .arg("--manifest")
        .arg(manifest)
        .arg("--cert")
        .arg(cert)
        .arg("--policy")
        .arg(policy)
        .status()
        .map_err(|e| format!("failed to launch verifier {signer_bin}: {e}"))?;
    if !status.success() {
        return Err("manifest verification failed; refusing install".into());
    }
    Ok(())
}
