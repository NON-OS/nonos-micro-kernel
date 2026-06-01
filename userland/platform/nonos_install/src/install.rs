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

use std::fs;
use std::path::Path;

use nonos_package::{package_dir, Package};

use super::verify::verify;
use super::verify_payload::verify_payload;

pub fn install(
    signer_bin: &str,
    pkg: &Package,
    cert: &Path,
    policy: &Path,
    store: &Path,
) -> Result<(), String> {
    verify(signer_bin, &pkg.manifest, cert, policy)?;
    verify_payload(&pkg.manifest, &pkg.payload)?;
    let (a, b, c) = pkg.version;
    let dir = package_dir(store, &pkg.name, pkg.version);
    fs::create_dir_all(&dir).map_err(|e| format!("create store dir: {e}"))?;
    fs::copy(&pkg.payload, dir.join("payload.elf")).map_err(|e| format!("copy payload: {e}"))?;
    fs::copy(&pkg.manifest, dir.join("manifest.nmf")).map_err(|e| format!("copy manifest: {e}"))?;
    let meta = format!("{}\n{a}.{b}.{c}\n{}\n", pkg.name, pkg.target);
    fs::write(dir.join("package.meta"), meta).map_err(|e| format!("write meta: {e}"))?;
    Ok(())
}
