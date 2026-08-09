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

mod common;

use common::{canonical, ELF, IDCERT, MANIFEST, ZKTR};
use nonos_pack_core::parse;

#[test]
fn canonical_package_parses_all_four_sections() {
    let b = canonical();
    let (s, trailer_off) = parse(&b).unwrap();
    assert_eq!(s.manifest, MANIFEST);
    assert_eq!(s.elf, ELF);
    assert_eq!(s.id_cert, IDCERT);
    assert_eq!(s.zk_trailer, ZKTR);
    assert_eq!(trailer_off, b.len());
}

#[test]
fn trailer_off_stops_at_signed_region_end() {
    let plain_len = canonical().len();
    let mut b = canonical();
    b.extend_from_slice(&common::trailer(&[(1, &[0xAA; 64]), (2, &[0xBB; 3309])]));
    let (_, trailer_off) = parse(&b).unwrap();
    assert_eq!(trailer_off, plain_len);
}

#[test]
fn empty_zk_trailer_section_is_valid() {
    let b = common::pack(&[(1, MANIFEST), (2, ELF), (3, IDCERT), (4, b"")]);
    let (s, _) = parse(&b).unwrap();
    assert_eq!(s.zk_trailer, b"");
}
