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

use common::{canonical, trailer};
use nonos_pack_core::{check_trailer, PkgErr};

fn sealed(t: &[u8]) -> Vec<u8> {
    let mut b = canonical();
    b.extend_from_slice(t);
    b
}

#[test]
fn canonical_two_sig_trailer_passes() {
    let off = canonical().len();
    let b = sealed(&trailer(&[(1, &[0xAA; 64]), (2, &[0xBB; 3309])]));
    assert_eq!(check_trailer(&b, off), Ok(()));
}

#[test]
fn missing_trailer_is_bad_trailer() {
    let b = canonical();
    assert_eq!(check_trailer(&b, b.len()), Err(PkgErr::BadTrailer));
}

#[test]
fn single_signature_is_bad_trailer() {
    let off = canonical().len();
    let b = sealed(&trailer(&[(1, &[0xAA; 64])]));
    assert_eq!(check_trailer(&b, off), Err(PkgErr::BadTrailer));
}

#[test]
fn swapped_tags_are_bad_trailer() {
    let off = canonical().len();
    let b = sealed(&trailer(&[(2, &[0xBB; 3309]), (1, &[0xAA; 64])]));
    assert_eq!(check_trailer(&b, off), Err(PkgErr::BadTrailer));
}

#[test]
fn trailing_garbage_is_bad_trailer() {
    let off = canonical().len();
    let mut b = sealed(&trailer(&[(1, &[0xAA; 64]), (2, &[0xBB; 3309])]));
    b.push(0x00);
    assert_eq!(check_trailer(&b, off), Err(PkgErr::BadTrailer));
}

#[test]
fn truncated_signature_is_bad_trailer() {
    let off = canonical().len();
    let full = sealed(&trailer(&[(1, &[0xAA; 64]), (2, &[0xBB; 3309])]));
    assert_eq!(check_trailer(&full[..full.len() - 1], off), Err(PkgErr::BadTrailer));
}

#[test]
fn offset_past_end_is_bad_trailer() {
    let b = canonical();
    assert_eq!(check_trailer(&b, b.len() + 1), Err(PkgErr::BadTrailer));
}
