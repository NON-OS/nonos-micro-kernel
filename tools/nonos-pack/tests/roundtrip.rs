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

use nonos_pack::container::{decode, encode_unsigned, Container, Section, SectionKind};

#[test]
fn roundtrip_preserves_all_four_sections() {
    let c = Container {
        sections: vec![
            Section { kind: SectionKind::Manifest, bytes: vec![3u8; 40] },
            Section { kind: SectionKind::Elf, bytes: vec![7u8; 4096] },
            Section { kind: SectionKind::IdCert, bytes: vec![9u8; 128] },
            Section { kind: SectionKind::ZkTrailer, bytes: vec![] },
        ],
    };
    let bytes = encode_unsigned(&c);
    let (back, trailer_off) = decode(&bytes).unwrap();
    assert_eq!(trailer_off, bytes.len());
    for k in [
        SectionKind::Manifest,
        SectionKind::Elf,
        SectionKind::IdCert,
        SectionKind::ZkTrailer,
    ] {
        assert_eq!(
            nonos_pack::container::section(&back, k),
            nonos_pack::container::section(&c, k)
        );
    }
}

#[test]
fn decode_rejects_bad_magic() {
    let mut bytes = encode_unsigned(&Container {
        sections: vec![
            Section { kind: SectionKind::Manifest, bytes: vec![1] },
            Section { kind: SectionKind::Elf, bytes: vec![1] },
            Section { kind: SectionKind::IdCert, bytes: vec![1] },
            Section { kind: SectionKind::ZkTrailer, bytes: vec![] },
        ],
    });
    bytes[0] = b'X';
    assert!(decode(&bytes).is_err());
}
