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

//! The report descriptor, read over the bus and parsed into the field map
//! every frame is decoded through. A field off by a bit reads as a finger
//! that jumps or a tip that never lifts.

use nonos_i2cmodel::touchpad::{self, MODE_REPORT_ID, TOUCH_REPORT_ID, X_MAX, Y_MAX};

use super::fixture::rig;
use crate::hid::{parse_report_descriptor, TouchLayout};
use crate::setup;

/// (bit offset, bit size, logical maximum) of each input field, in report
/// order.
fn placed(l: &TouchLayout) -> [(u32, u32, i32); 6] {
    [
        (l.confidence.bit_offset, l.confidence.bit_size, l.confidence.logical_max),
        (l.tip.bit_offset, l.tip.bit_size, l.tip.logical_max),
        (l.x.bit_offset, l.x.bit_size, l.x.logical_max),
        (l.y.bit_offset, l.y.bit_size, l.y.logical_max),
        (l.contact_count.bit_offset, l.contact_count.bit_size, l.contact_count.logical_max),
        (l.button.bit_offset, l.button.bit_size, l.button.logical_max),
    ]
}

const EXPECTED: [(u32, u32, i32); 6] =
    [(0, 1, 1), (1, 1, 1), (8, 16, X_MAX as i32), (24, 16, Y_MAX as i32), (40, 8, 5), (48, 1, 1)];

#[test]
fn the_parser_places_every_input_field_where_the_descriptor_puts_it() {
    let layout = parse_report_descriptor(&touchpad::report_descriptor());
    assert_eq!(layout.report_id, TOUCH_REPORT_ID);
    assert_eq!(placed(&layout), EXPECTED);
    assert!(layout.is_absolute_touch());
}

#[test]
fn the_feature_report_fields_are_found_under_their_own_report_id() {
    let l = parse_report_descriptor(&touchpad::report_descriptor());
    assert_eq!((l.input_mode.bit_offset, l.input_mode.bit_size), (0, 8));
    assert_eq!((l.surface_switch.bit_offset, l.surface_switch.bit_size), (8, 1));
    assert_eq!((l.button_switch.bit_offset, l.button_switch.bit_size), (9, 1));
    let ids = [l.input_mode_report_id, l.surface_switch_report_id, l.button_switch_report_id];
    assert_eq!(ids, [MODE_REPORT_ID; 3]);
}

#[test]
fn setup_reads_the_whole_descriptor_the_pad_declares_and_parses_the_same_map() {
    let _r = rig();
    let state = setup::run().expect("setup");
    assert_eq!(placed(&state.touch_layout), EXPECTED, "layout differs when read over the bus");
}
