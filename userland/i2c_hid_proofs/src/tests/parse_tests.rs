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

//! The fallback decode for a pad that reports as a boot mouse: length
//! prefix, optional report id, buttons, dx, dy, wheel.

use crate::input::parse_report::parse_report;

#[test]
fn a_boot_mouse_frame_without_a_report_id_decodes_in_place() {
    let frame = [6, 0, 0x00, 5, 0xFD, 0];
    let s = parse_report(&frame).expect("a well-formed frame");
    assert_eq!((s.buttons, s.dx, s.dy, s.wheel), (0, 5, -3, 0));
}

#[test]
fn a_report_id_in_front_of_the_buttons_is_skipped() {
    let frame = [7, 0, 0x01, 0x01, 5, 0xFD, 2];
    let s = parse_report(&frame).expect("a well-formed frame");
    assert_eq!((s.buttons, s.dx, s.dy, s.wheel), (1, 5, -3, 2));
}

#[test]
fn a_length_prefix_longer_than_the_buffer_is_refused_rather_than_read_past() {
    assert!(parse_report(&[9, 0, 0, 1, 2]).is_none());
}

#[test]
fn a_frame_too_short_to_carry_a_sample_is_nothing() {
    assert!(parse_report(&[2, 0]).is_none(), "the zero-length reset report decoded as motion");
    assert!(parse_report(&[4, 0, 0, 1]).is_none());
}
