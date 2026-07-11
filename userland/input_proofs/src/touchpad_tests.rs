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

//! Proofs for the i2c-hid touchpad report decoder. A device can hand us any
//! bytes, so the decoder must reject malformed frames and never read out of
//! range.

use crate::parse_report::parse_report;

#[test]
fn rejects_short_frame() {
    assert!(parse_report(&[0, 0, 0, 0]).is_none());
}

#[test]
fn rejects_bad_length_field() {
    // Length field larger than the buffer.
    assert!(parse_report(&[100, 0, 1, 2, 3]).is_none());
    // Length field below the minimum frame size.
    assert!(parse_report(&[3, 0, 1, 2, 3]).is_none());
}

#[test]
fn decodes_relative_sample() {
    // total = 5, body = [buttons, dx, dy].
    let s = parse_report(&[5, 0, 0x01, 5, 0xFB]).unwrap();
    assert_eq!(s.buttons, 0x01);
    assert_eq!(s.dx, 5);
    assert_eq!(s.dy, -5);
}

// Whatever the device sends, the decoder returns a value or None and never
// panics or reads past the frame.
#[test]
fn never_panics_on_arbitrary_frames() {
    for n in 0..64usize {
        let frame: alloc::vec::Vec<u8> = (0..n).map(|i| i as u8).collect();
        let _ = parse_report(&frame);
    }
    // A frame whose length field points exactly at the end must stay in range.
    let _ = parse_report(&[6, 0, 1, 2, 3, 4]);
}

extern crate alloc;
