// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use std::{ffi::OsString, path::Path};

use super::run_output::run_output;

pub fn generate_proof(
    tool: &Path,
    pk: &Path,
    cap: &Path,
    caps: &str,
    out: &Path,
) -> Result<String, String> {
    let args = [
        OsString::from("--proving-key"),
        pk.as_os_str().to_os_string(),
        OsString::from("--capsule"),
        cap.as_os_str().to_os_string(),
        OsString::from("--capability-mask"),
        OsString::from(caps),
        OsString::from("--output"),
        out.join("cap.proof").into_os_string(),
        OsString::from("--public-inputs-out"),
        out.join("cap.pubins").into_os_string(),
        OsString::from("--trailer-out"),
        out.join("cap.trailer").into_os_string(),
        OsString::from("--capsule-with-trailer-out"),
        out.join("term.cap").into_os_string(),
    ];
    run_output(tool, &args)
}
