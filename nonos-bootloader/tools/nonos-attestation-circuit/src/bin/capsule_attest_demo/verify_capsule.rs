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

use super::run_status::run_status;

pub fn verify_capsule(tool: &Path, vk: &Path, capsule: &Path) -> Result<(bool, String), String> {
    let args = [
        OsString::from("--verifying-key"),
        vk.as_os_str().to_os_string(),
        OsString::from("--capsule"),
        capsule.as_os_str().to_os_string(),
    ];
    run_status(tool, &args)
}
