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

use nonos_pack::container::{Container, Section, SectionKind};
use nonos_pack::sign::seal;

use super::args::{flag, seeds};

const INPUTS: [(SectionKind, &str); 4] = [
    (SectionKind::Manifest, "--manifest"),
    (SectionKind::Elf, "--elf"),
    (SectionKind::IdCert, "--id-cert"),
    (SectionKind::ZkTrailer, "--trailer"),
];

pub fn run(av: &[String]) -> Result<(), String> {
    let out = flag("pack", av, "--out")?;
    let mut sections = Vec::with_capacity(INPUTS.len());
    for (kind, name) in INPUTS {
        let path = flag("pack", av, name)?;
        let bytes = fs::read(&path).map_err(|e| format!("{}: {}", path, e))?;
        sections.push(Section { kind, bytes });
    }
    let (ed, mldsa) = seeds("pack", av)?;
    let bytes = seal(&Container { sections }, &ed, &mldsa)
        .map_err(|e| format!("pack: seal failed: {:?}", e))?;
    fs::write(&out, &bytes).map_err(|e| format!("{}: {}", out, e))?;
    println!("wrote {} ({} bytes)", out, bytes.len());
    Ok(())
}
