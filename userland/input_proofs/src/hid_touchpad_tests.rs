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

//! Proofs for the HID report-descriptor parser and the absolute-touch decoder,
//! driven by a realistic precision-touchpad descriptor and report.

use crate::decode::decode_touch;
use crate::parse::parse;

// A single-finger precision-touchpad report descriptor: report id 1, a tip
// switch, 7 padding bits, 12-bit-range X and Y (16-bit fields), an 8-bit
// contact count, and a clickpad button.
const TOUCHPAD_DESC: &[u8] = &[
    0x05, 0x0D, // Usage Page (Digitizer)
    0x09, 0x05, // Usage (Touch Pad)
    0xA1, 0x01, // Collection (Application)
    0x85, 0x01, //   Report ID (1)
    0x09, 0x22, //   Usage (Finger)
    0xA1, 0x02, //   Collection (Logical)
    0x05, 0x0D, //     Usage Page (Digitizer)
    0x09, 0x42, //     Usage (Tip Switch)
    0x25, 0x01, //     Logical Maximum (1)
    0x75, 0x01, //     Report Size (1)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x02, //     Input (Data,Var,Abs)
    0x95, 0x07, //     Report Count (7)
    0x81, 0x03, //     Input (Const)  padding
    0x05, 0x01, //     Usage Page (Generic Desktop)
    0x26, 0xFF, 0x0F, // Logical Maximum (4095)
    0x75, 0x10, //     Report Size (16)
    0x95, 0x01, //     Report Count (1)
    0x09, 0x30, //     Usage (X)
    0x81, 0x02, //     Input
    0x09, 0x31, //     Usage (Y)
    0x81, 0x02, //     Input
    0xC0, //   End Collection
    0x05, 0x0D, //   Usage Page (Digitizer)
    0x09, 0x54, //   Usage (Contact Count)
    0x25, 0x7F, //   Logical Maximum (127)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x02, //   Input
    0x05, 0x09, //   Usage Page (Button)
    0x09, 0x01, //   Usage (Button 1)
    0x25, 0x01, //   Logical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x02, //   Input
    0x95, 0x07, //   Report Count (7)
    0x81, 0x03, //   Input (Const)  padding
    0xC0, // End Collection
];

#[test]
fn parses_touchpad_field_map() {
    let layout = parse(TOUCHPAD_DESC);
    assert!(layout.is_absolute_touch());
    assert_eq!(layout.report_id, 1);
    assert_eq!((layout.x.bit_offset, layout.x.bit_size, layout.x.logical_max), (8, 16, 4095));
    assert_eq!((layout.y.bit_offset, layout.y.bit_size, layout.y.logical_max), (24, 16, 4095));
    assert_eq!((layout.tip.bit_offset, layout.tip.bit_size), (0, 1));
    assert_eq!((layout.contact_count.bit_offset, layout.contact_count.bit_size), (40, 8));
    assert_eq!((layout.button.bit_offset, layout.button.bit_size), (48, 1));
}

#[test]
fn decodes_absolute_contact() {
    let layout = parse(TOUCHPAD_DESC);
    // report id 1, tip=1, X=1000 (0x03E8), Y=500 (0x01F4), contacts=1, button=0.
    let report = [0x01, 0x01, 0xE8, 0x03, 0xF4, 0x01, 0x01, 0x00];
    let s = decode_touch(&report, &layout).unwrap();
    assert!(s.tip);
    assert_eq!(s.x, 1000);
    assert_eq!(s.y, 500);
    assert_eq!(s.contacts, 1);
    assert!(!s.button);
    assert_eq!((s.x_max, s.y_max), (4095, 4095));
}

#[test]
fn rejects_wrong_report_id() {
    let layout = parse(TOUCHPAD_DESC);
    // Report id 2 is not the touch report; must not be decoded as one.
    let report = [0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    assert!(decode_touch(&report, &layout).is_none());
}

#[test]
fn short_report_never_reads_out_of_range() {
    let layout = parse(TOUCHPAD_DESC);
    for n in 0..10usize {
        let frame = vec![0x01u8; n];
        let _ = decode_touch(&frame, &layout);
    }
}

// A garbage descriptor yields an unusable layout rather than a panic.
#[test]
fn garbage_descriptor_is_not_absolute_touch() {
    assert!(!parse(&[0xFF, 0xFF, 0xFF, 0x00, 0x11]).is_absolute_touch());
    assert!(!parse(&[]).is_absolute_touch());
}
