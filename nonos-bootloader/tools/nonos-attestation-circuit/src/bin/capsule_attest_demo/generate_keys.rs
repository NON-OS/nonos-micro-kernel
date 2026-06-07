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

pub fn generate_keys(tool: &Path, dir: &Path) -> Result<(), String> {
    let args = [
        OsString::from("generate"),
        OsString::from("--output"),
        dir.as_os_str().to_os_string(),
        OsString::from("--allow-unsigned"),
    ];
    run_output(tool, &args).map(|_| ())
}
