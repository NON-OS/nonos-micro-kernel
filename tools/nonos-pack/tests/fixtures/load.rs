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

use nonos_capsule_sign::algs::AlgId;
use nonos_capsule_sign::keys::read_seed;
use nonos_capsule_sign::sign::sign_with;
use nonos_pack::container::{encode_unsigned, Container, Section, SectionKind};

use super::paths::gui_demo_paths;

pub fn gui_demo_container_and_seeds() -> Option<(Container, Vec<u8>, Vec<u8>)> {
    let p = gui_demo_paths()?;
    let c = Container {
        sections: vec![
            Section { kind: SectionKind::Manifest, bytes: fs::read(&p.manifest).ok()? },
            Section { kind: SectionKind::Elf, bytes: fs::read(&p.elf).ok()? },
            Section { kind: SectionKind::IdCert, bytes: fs::read(&p.id_cert).ok()? },
            Section { kind: SectionKind::ZkTrailer, bytes: fs::read(&p.trailer).ok()? },
        ],
    };
    let ed = read_seed(&p.ed_seed).ok()?;
    let mldsa = read_seed(&p.mldsa_seed).ok()?;
    Some((c, ed.bytes, mldsa.bytes))
}

pub fn first_elf_byte_offset(sealed: &[u8]) -> usize {
    let count = u16::from_be_bytes([sealed[6], sealed[7]]) as usize;
    for i in 0..count {
        let e = &sealed[8 + i * 16..8 + (i + 1) * 16];
        if u16::from_be_bytes([e[0], e[1]]) == SectionKind::Elf as u16 {
            return u32::from_be_bytes([e[4], e[5], e[6], e[7]]) as usize;
        }
    }
    panic!("sealed package has no Elf section");
}

pub fn seal_ed25519_only(c: &Container, ed_seed: &[u8]) -> Vec<u8> {
    let mut out = encode_unsigned(c);
    let digest = blake3::hash(&out);
    let sig = sign_with(AlgId::Ed25519, ed_seed, digest.as_bytes()).unwrap();
    out.push(1);
    out.push(1);
    out.extend_from_slice(&(sig.len() as u16).to_be_bytes());
    out.extend_from_slice(&sig);
    out
}
