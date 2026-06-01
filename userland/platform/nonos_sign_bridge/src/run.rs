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

use std::process::Command;

pub fn run_signer(signer_bin: &str, args: &[String]) -> Result<(), String> {
    let status = Command::new(signer_bin)
        .args(args)
        .status()
        .map_err(|e| format!("failed to launch signer {signer_bin}: {e}"))?;
    if !status.success() {
        return Err(format!("signer exited with {status}"));
    }
    Ok(())
}
