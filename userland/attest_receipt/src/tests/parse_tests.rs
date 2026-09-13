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

//! Reading the fields back out of bytes from a machine.

use super::fixtures::{flatten, three_capsules, BROWSER, KEYRING};
use crate::decode::Authority;
use crate::entries::{parse_entries, ParseError};

#[test]
fn the_fields_come_back_the_way_the_kernel_wrote_them() {
    let parsed = parse_entries(&flatten(&three_capsules())).expect("a whole set parses");
    assert_eq!(parsed[0].pid, 12);
    assert_eq!(parsed[0].caps, KEYRING);
    assert_eq!(parsed[0].measurement, [0xAA; 32]);
    assert_eq!(parsed[0].authority, Authority::Vendor);
    assert_eq!(parsed[2].caps, BROWSER);
}

#[test]
fn a_truncated_set_is_refused_rather_than_rounded_down() {
    let mut bytes = flatten(&three_capsules());
    bytes.truncate(bytes.len() - 7);
    assert_eq!(parse_entries(&bytes), Err(ParseError::Ragged { len: bytes.len() }));
}

#[test]
fn an_empty_set_parses_to_nothing_rather_than_failing() {
    assert_eq!(parse_entries(&[]), Ok(Vec::new()));
}

#[test]
fn the_authority_byte_inverts_the_kernels_encoding() {
    assert_eq!(Authority::from_byte(0), Authority::Vendor);
    assert_eq!(Authority::from_byte(1), Authority::Developer(0));
    assert_eq!(Authority::from_byte(4), Authority::Developer(3));
    assert_eq!(Authority::from_byte(255), Authority::Publisher);
}
