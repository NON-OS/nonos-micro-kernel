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

use nonos_manifest::cap_mask;

use crate::util::load_manifest;

pub fn run(args: &[String]) -> Result<(), String> {
    let dir = args.first().map(String::as_str).unwrap_or(".");
    let m = load_manifest(Path::new(dir))?;
    let required = cap_mask(&m.required_caps)?;
    let optional = cap_mask(&m.optional_caps)?;
    let (a, b, c) = m.version;
    println!("name        {}", m.name);
    println!("namespace   {}", m.namespace);
    println!("version     {a}.{b}.{c}");
    println!("target      {}", m.target);
    println!("cert        {}", m.cert);
    println!("required    0x{required:x}  [{}]", m.required_caps.join(", "));
    println!("optional    0x{optional:x}");
    println!("endpoints   {}", m.endpoints.len());
    Ok(())
}
