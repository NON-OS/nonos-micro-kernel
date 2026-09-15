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

//! The parser against responses shaped like the ones a real part gives.

use super::fixtures::{response, OUT_PUBLIC_AT};
use crate::security::tpm::ak::parse_public;
use crate::security::tpm::error::TpmError;

#[test]
fn the_point_comes_out_x_then_y() {
    let point = parse_public(&response(0x0003, 0x0018, 32, 32)).expect("parses");
    assert!(point[..32].iter().all(|b| *b == 0xAA));
    assert!(point[32..].iter().all(|b| *b == 0xBB));
}

#[test]
fn another_curve_is_not_this_kernels_key() {
    assert_eq!(parse_public(&response(0x0004, 0x0018, 32, 32)), Err(TpmError::InvalidResponse));
}

#[test]
fn a_key_without_the_ecdsa_scheme_is_refused() {
    assert_eq!(parse_public(&response(0x0003, 0x0010, 32, 32)), Err(TpmError::InvalidResponse));
}

#[test]
fn a_coordinate_of_the_wrong_length_is_refused() {
    assert_eq!(parse_public(&response(0x0003, 0x0018, 31, 32)), Err(TpmError::InvalidResponse));
    assert_eq!(parse_public(&response(0x0003, 0x0018, 32, 33)), Err(TpmError::InvalidResponse));
}

#[test]
fn a_response_cut_inside_the_point_is_refused_not_read_short() {
    let r = response(0x0003, 0x0018, 32, 32);
    for cut in [OUT_PUBLIC_AT + 1, r.len() - 20, r.len() - 9] {
        assert_eq!(parse_public(&r[..cut]), Err(TpmError::InvalidResponse), "cut at {cut}");
    }
}
