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

use nonos_sign_bridge::sign_manifest;

use crate::env::signer_bin;
use crate::util::load_manifest;

pub fn run(args: &[String]) -> Result<(), String> {
    let dir = args.first().map(String::as_str).unwrap_or(".");
    let project = Path::new(dir);
    let m = load_manifest(project)?;
    let elf = project.join("target").join(&m.target).join("release").join(&m.name);
    if !elf.exists() {
        return Err(format!("payload not built: {} (run `build` first)", elf.display()));
    }
    let dist = project.join("dist");
    fs::create_dir_all(&dist).map_err(|e| format!("mkdir dist: {e}"))?;
    let nmf = dist.join("manifest.nmf");
    let mut model = m.clone();
    model.cert = project.join(&m.cert).to_string_lossy().into_owned();
    model.pub_seeds = m
        .pub_seeds
        .iter()
        .map(|(a, p)| (a.clone(), project.join(p).to_string_lossy().into_owned()))
        .collect();
    sign_manifest(&signer_bin(), &model, &elf.to_string_lossy(), &nmf.to_string_lossy())?;
    fs::copy(&elf, dist.join("payload.elf")).map_err(|e| format!("copy payload: {e}"))?;
    let (a, b, c) = m.version;
    let meta = format!("{}\n{a}.{b}.{c}\n{}\n{}\n", m.name, m.target, m.namespace);
    fs::write(dist.join("package.meta"), meta).map_err(|e| format!("write meta: {e}"))?;
    println!("signed package -> {}", dist.display());
    Ok(())
}
