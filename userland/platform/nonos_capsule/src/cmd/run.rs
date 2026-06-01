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

use nonos_install::{verify, verify_payload};
use nonos_package::package_dir;
use nonos_registry::find;

use crate::env::{index_path, signer_bin, store_dir};
use crate::util::flag;

pub fn run(args: &[String]) -> Result<(), String> {
    let name = args.first().ok_or("usage: run <name> --cert <c> --policy <p>")?;
    let entry = find(&index_path(), name).ok_or_else(|| format!("not installed: {name}"))?;
    let dir = package_dir(&store_dir(), &entry.name, entry.version);
    let manifest = dir.join("manifest.nmf");
    let payload = dir.join("payload.elf");
    let cert = flag(args, "--cert").ok_or("missing --cert <path>")?;
    let policy = flag(args, "--policy").ok_or("missing --policy <path>")?;
    verify(&signer_bin(), &manifest, Path::new(&cert), Path::new(&policy))?;
    verify_payload(&manifest, &payload)?;
    let (a, b, c) = entry.version;
    println!("{name} {a}.{b}.{c} verified and spawn-ready at {}", dir.display());
    println!("spawn is performed by the kernel verified-spawn path at load time");
    Ok(())
}
