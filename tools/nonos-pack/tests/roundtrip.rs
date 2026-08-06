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

mod fixtures;

use nonos_pack::container::{decode, encode_unsigned, Container, PackErr, Section, SectionKind};

macro_rules! gui_demo_or_skip {
    () => {
        match fixtures::gui_demo_container_and_seeds() {
            Some(f) => f,
            None => {
                eprintln!("skip: gui_demo artifacts or publisher seeds absent");
                return;
            }
        }
    };
}

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

#[test]
fn seal_then_verify_roundtrips() {
    let (c, ed_seed, mldsa_seed) = gui_demo_or_skip!();
    let sealed = nonos_pack::sign::seal(&c, &ed_seed, &mldsa_seed).unwrap();
    nonos_pack::sign::verify(&sealed).expect("freshly sealed package must verify");
}

#[test]
fn verify_rejects_tampered_elf() {
    let (c, ed_seed, mldsa_seed) = gui_demo_or_skip!();
    let mut sealed = nonos_pack::sign::seal(&c, &ed_seed, &mldsa_seed).unwrap();
    let elf_off = fixtures::first_elf_byte_offset(&sealed);
    sealed[elf_off] ^= 0xFF;
    assert!(nonos_pack::sign::verify(&sealed).is_err());
}

#[test]
fn verify_rejects_missing_mldsa_signature() {
    let (c, ed_seed, _) = gui_demo_or_skip!();
    let ed_only = fixtures::seal_ed25519_only(&c, &ed_seed);
    assert!(nonos_pack::sign::verify(&ed_only).is_err());
}

#[test]
fn verify_rejects_corrupt_mldsa_signature() {
    let (c, ed_seed, mldsa_seed) = gui_demo_or_skip!();
    let sealed = nonos_pack::sign::seal(&c, &ed_seed, &mldsa_seed).unwrap();
    let bad = fixtures::corrupt_signature(&sealed, 2);
    assert!(matches!(nonos_pack::sign::verify(&bad), Err(PackErr::BadSignature("mldsa65"))));
}

#[test]
fn verify_rejects_appended_trailer_entry() {
    let (c, ed_seed, mldsa_seed) = gui_demo_or_skip!();
    let sealed = nonos_pack::sign::seal(&c, &ed_seed, &mldsa_seed).unwrap();
    let padded = fixtures::append_trailer_entry(&sealed, 0x7f, &[0xAAu8; 4096]);
    assert!(matches!(nonos_pack::sign::verify(&padded), Err(PackErr::NonCanonicalTrailer)));
}

#[test]
fn verify_rejects_duplicate_ed25519_entry() {
    let (c, ed_seed, mldsa_seed) = gui_demo_or_skip!();
    let sealed = nonos_pack::sign::seal(&c, &ed_seed, &mldsa_seed).unwrap();
    let dup = fixtures::append_trailer_entry(&sealed, 1, &[0u8; 64]);
    assert!(matches!(nonos_pack::sign::verify(&dup), Err(PackErr::NonCanonicalTrailer)));
}
