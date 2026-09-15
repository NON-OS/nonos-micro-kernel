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

//! Where each stream descriptor lives, which GCAP alone decides.
//!
//! The specification packs the descriptor blocks in a fixed order, inputs
//! first, then outputs, then bidirectional, and a controller only says how
//! many of each it has. Counting them wrong does not fail loudly: it aims the
//! playback writes at an input engine, which accepts every register and never
//! produces a sample, so the machine looks like it has no speakers.

use nonos_devmodel::FakeBar;

use crate::controller::streams::{addr64, bidi_streams, input_streams, output_streams};
use crate::controller::{layout, ControllerInfo, STREAM_OUTPUT};
use crate::model::WINDOW;
use crate::regs::Regs;

/// A four-in, four-out controller that can address 64-bit memory, which is
/// what every Intel PCH this system has booted on reports.
const GCAP_INTEL_PCH: u16 = 0x4401;
/// Every count saturated: 15 in, 15 out, 31 bidirectional.
const GCAP_MAX: u16 = 0xffff;

fn info(gcap: u16) -> ControllerInfo {
    let bar = FakeBar::new(WINDOW);
    bar.present16(0x00, gcap);
    ControllerInfo::read(Regs::new(bar.base()))
}

#[test]
fn gcap_is_decoded_into_the_fields_the_specification_places_there() {
    assert_eq!((input_streams(GCAP_INTEL_PCH), output_streams(GCAP_INTEL_PCH)), (4, 4));
    assert_eq!((bidi_streams(GCAP_INTEL_PCH), addr64(GCAP_INTEL_PCH)), (0, 1));
    assert_eq!((input_streams(GCAP_MAX), output_streams(GCAP_MAX)), (15, 15));
    assert_eq!((bidi_streams(GCAP_MAX), addr64(GCAP_MAX)), (31, 1));
    /*
     * A controller with no 64-bit support must not be read as having it, or
     * the upper half of every ring address is programmed and ignored.
     */
    assert_eq!(addr64(GCAP_INTEL_PCH & !1), 0);
}

#[test]
fn the_output_descriptors_begin_after_every_input_descriptor() {
    let (descs, n) = layout(info(GCAP_INTEL_PCH));
    assert_eq!(n, 8, "four inputs and four outputs are eight descriptors");
    assert_eq!(descs[4].kind, STREAM_OUTPUT, "the fifth block is the first output engine");
    assert_eq!(descs[4].mmio_offset, 0x100);
    assert_eq!(descs[4].local_index, 0, "it is the controller's output stream zero");
}

#[test]
fn every_descriptor_owns_a_distinct_block_inside_the_smallest_legal_bar() {
    let (descs, n) = layout(info(GCAP_MAX));
    assert_eq!(n, 61);
    for (i, d) in descs.iter().take(n).enumerate() {
        assert_eq!(d.global_index as usize, i, "descriptor {i} is misnumbered");
        assert_eq!(d.mmio_offset, 0x80 + i as u32 * 0x20, "descriptor {i} overlaps its neighbour");
        assert!(d.mmio_offset as usize + 0x20 <= WINDOW, "descriptor {i} falls outside the BAR");
    }
}
