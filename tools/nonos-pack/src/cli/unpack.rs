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

use nonos_pack::container::{decode, SectionKind};

use super::args::flag;

fn suffix(k: SectionKind) -> &'static str {
    match k {
        SectionKind::Manifest => ".manifest.bin",
        SectionKind::Elf => ".elf",
        SectionKind::IdCert => ".nonos_id_cert.bin",
        SectionKind::ZkTrailer => ".zk_trailer.bin",
    }
}

pub fn run(av: &[String]) -> Result<(), String> {
    let input = flag("unpack", av, "--in")?;
    let dir = flag("unpack", av, "--out-dir")?;
    let bytes = fs::read(&input).map_err(|e| format!("{}: {}", input, e))?;
    let (c, _) = decode(&bytes).map_err(|e| format!("unpack: {:?}", e))?;
    let base = Path::new(&input)
        .file_stem()
        .ok_or_else(|| format!("unpack: {} has no basename", input))?
        .to_string_lossy()
        .into_owned();
    fs::create_dir_all(&dir).map_err(|e| format!("{}: {}", dir, e))?;
    for s in &c.sections {
        let path = Path::new(&dir).join(format!("{}{}", base, suffix(s.kind)));
        fs::write(&path, &s.bytes).map_err(|e| format!("{}: {}", path.display(), e))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}
