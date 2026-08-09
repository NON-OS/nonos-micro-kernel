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

use super::types::{Container, PackErr, Section, SectionKind};

pub fn decode(bytes: &[u8]) -> Result<(Container, usize), PackErr> {
    let (s, trailer_off) = nonos_pack_core::parse(bytes).map_err(map_err)?;
    let sections = vec![
        Section { kind: SectionKind::Manifest, bytes: s.manifest.to_vec() },
        Section { kind: SectionKind::Elf, bytes: s.elf.to_vec() },
        Section { kind: SectionKind::IdCert, bytes: s.id_cert.to_vec() },
        Section { kind: SectionKind::ZkTrailer, bytes: s.zk_trailer.to_vec() },
    ];
    Ok((Container { sections }, trailer_off))
}

fn map_err(e: nonos_pack_core::PkgErr) -> PackErr {
    use nonos_pack_core::PkgErr as E;
    match e {
        E::BadMagic => PackErr::BadMagic,
        E::BadVersion => PackErr::BadVersion,
        E::DupKind => PackErr::Duplicate,
        E::OutOfOrder => PackErr::OutOfOrder,
        E::MissingSection | E::BadCount => PackErr::MissingSection,
        E::Short | E::BadExtent => PackErr::Truncated,
        E::BadTrailer => PackErr::NonCanonicalTrailer,
    }
}

pub fn section<'a>(c: &'a Container, k: SectionKind) -> Option<&'a [u8]> {
    c.sections.iter().find(|s| s.kind == k).map(|s| s.bytes.as_slice())
}
