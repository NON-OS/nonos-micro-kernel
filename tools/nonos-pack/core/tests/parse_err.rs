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

use common::{canonical, pack, ELF, IDCERT, MANIFEST, ZKTR};
use nonos_pack_core::{parse, PkgErr};

fn err(bytes: &[u8]) -> PkgErr {
    parse(bytes).map(|_| ()).unwrap_err()
}

#[test]
fn short_header_is_short() {
    assert_eq!(err(b"NOS1\x00"), PkgErr::Short);
}

#[test]
fn wrong_magic_is_bad_magic() {
    let mut b = canonical();
    b[0] = b'X';
    assert_eq!(err(&b), PkgErr::BadMagic);
}

#[test]
fn version_two_is_bad_version() {
    let mut b = canonical();
    b[5] = 2;
    assert_eq!(err(&b), PkgErr::BadVersion);
}

#[test]
fn three_sections_is_bad_count() {
    assert_eq!(err(&pack(&[(1, MANIFEST), (2, ELF), (3, IDCERT)])), PkgErr::BadCount);
}

#[test]
fn truncated_table_is_short() {
    assert_eq!(err(&canonical()[..8 + 32]), PkgErr::Short);
}

#[test]
fn duplicate_kind_is_dup_kind() {
    assert_eq!(err(&pack(&[(1, MANIFEST), (1, ELF), (3, IDCERT), (4, ZKTR)])), PkgErr::DupKind);
}

#[test]
fn descending_kind_is_out_of_order() {
    assert_eq!(err(&pack(&[(2, ELF), (1, MANIFEST), (3, IDCERT), (4, ZKTR)])), PkgErr::OutOfOrder);
}

#[test]
fn unknown_kind_is_missing_section() {
    assert_eq!(err(&pack(&[(1, MANIFEST), (2, ELF), (3, IDCERT), (5, ZKTR)])), PkgErr::MissingSection);
}

#[test]
fn section_past_end_is_bad_extent() {
    let b = canonical();
    assert_eq!(err(&b[..b.len() - 1]), PkgErr::BadExtent);
}
