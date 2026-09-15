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

//! Filling the framebuffer, and stopping where it ends.
//!
//! `clear` takes a bare user virtual address and a pixel count and writes one
//! 32-bit word per pixel with no bound of its own. On a booted machine the
//! address is a mapped framebuffer aperture and a count one too large runs
//! off the end of the mapping. A second window here stands in for that
//! aperture, so a test can watch the byte after the last pixel.

use nonos_devmodel::FakeBar;

use crate::dispi::clear;

const COLOR: u32 = 0x0010_2a3a;
const GUARD: u32 = 0xdead_beef;

/// An aperture of `pixels` pixels with one more pixel of guard behind it.
fn aperture(pixels: usize) -> FakeBar {
    let bar = FakeBar::new((pixels + 1) * 4);
    bar.present32(pixels * 4, GUARD);
    bar
}

#[test]
fn every_pixel_in_the_span_takes_the_colour_it_was_given() {
    /*
     * A clear that covers all but the last row is how a display comes up with
     * one band of whatever the firmware left behind, which reads as a corrupt
     * mode rather than as an arithmetic slip.
     */
    let pixels = 64;
    let fb = aperture(pixels);
    clear(fb.base(), pixels as u64, COLOR);

    for i in 0..pixels {
        assert_eq!(fb.wrote32(i * 4), COLOR, "pixel {i} was not cleared");
    }
}

#[test]
fn the_pixel_past_the_end_of_the_span_is_left_alone() {
    /*
     * The count is a pixel count, not an inclusive bound. One pixel of
     * overrun writes four bytes past the framebuffer mapping, which on a
     * booted machine faults the capsule or lands in whatever the pager put
     * next to the aperture. Neither points back at this loop.
     */
    let pixels = 64;
    let fb = aperture(pixels);
    clear(fb.base(), pixels as u64, COLOR);
    assert_eq!(fb.wrote32(pixels * 4), GUARD, "clear wrote past the last pixel");
}

#[test]
fn a_span_of_no_pixels_writes_nothing_at_all() {
    /*
     * A zero-sized mode is the degenerate case the discovery path can hand
     * over when a BAR fails to map, and it must not touch the aperture.
     */
    let fb = aperture(0);
    clear(fb.base(), 0, COLOR);
    assert_eq!(fb.wrote32(0), GUARD, "an empty clear still wrote a pixel");
}
