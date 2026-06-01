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

use nonos_package::package_dir;
use nonos_registry::find;

use crate::env::{index_path, store_dir};

pub fn run(args: &[String]) -> Result<(), String> {
    let name = args.first().ok_or("usage: inspect <name>")?;
    let entry = find(&index_path(), name).ok_or_else(|| format!("not installed: {name}"))?;
    let (a, b, c) = entry.version;
    let dir = package_dir(&store_dir(), &entry.name, entry.version);
    println!("name          {}", entry.name);
    println!("version       {a}.{b}.{c}");
    println!("target        {}", entry.target);
    println!("namespace     {}", entry.namespace);
    println!("publisher fp  {}", entry.cert_fingerprint);
    println!("store path    {}", dir.display());
    println!("manifest      {}", dir.join("manifest.nmf").display());
    Ok(())
}
