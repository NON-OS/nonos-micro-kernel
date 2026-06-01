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

use nonos_install::install;
use nonos_package::Package;
use nonos_registry::{add, RegistryEntry};

use crate::env::{index_path, signer_bin, store_dir};
use crate::util::{fingerprint, flag, read_meta};

pub fn run(args: &[String]) -> Result<(), String> {
    let pkg = args.first().ok_or("usage: install <package-dir> --cert <c> --policy <p>")?;
    let cert = flag(args, "--cert").ok_or("missing --cert <path>")?;
    let policy = flag(args, "--policy").ok_or("missing --policy <path>")?;
    let dir = Path::new(pkg);
    let (name, version, target, namespace) = read_meta(dir)?;
    let package = Package {
        name: name.clone(),
        version,
        target: target.clone(),
        payload: dir.join("payload.elf"),
        manifest: dir.join("manifest.nmf"),
    };
    install(&signer_bin(), &package, Path::new(&cert), Path::new(&policy), &store_dir())?;
    add(
        &index_path(),
        RegistryEntry { name: name.clone(), version, target, namespace,
            cert_fingerprint: fingerprint(Path::new(&cert)) },
    )?;
    let (a, b, c) = version;
    println!("installed {name} {a}.{b}.{c}");
    Ok(())
}
