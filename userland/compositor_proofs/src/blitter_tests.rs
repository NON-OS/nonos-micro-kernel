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

//! The blitter writes raw pixels through pointers, so the property that matters
//! is containment: a fill or a composite must never touch a byte outside the
//! destination surface, no matter how the source rectangle, clip, or layer
//! placement is chosen. We back a real `Surface` with a guarded heap buffer and
//! prove the guard bytes are untouched, plus that clipping is exact.

use crate::damage::Rect;
use crate::sw_blitter::{composite_layer, fill_rect, Surface};

const W: u32 = 40;
const H: u32 = 30;
const GUARD: usize = 16;

// A destination buffer with guard cells before and after the real surface, so
// any out-of-bounds write lands in a guard and is caught.
struct Canvas {
    buf: Vec<u32>,
}

impl Canvas {
    fn new() -> Self {
        Canvas { buf: vec![0u32; GUARD + (W * H) as usize + GUARD] }
    }
    fn surface(&mut self) -> Surface {
        let base = unsafe { self.buf.as_mut_ptr().add(GUARD) } as u64;
        Surface { base_va: base, stride: W * 4, width: W, height: H, byte_len: (W * H * 4) as u64 }
    }
    fn assert_guards_intact(&self) {
        for i in 0..GUARD {
            assert_eq!(self.buf[i], 0, "front guard {i} clobbered");
            assert_eq!(self.buf[self.buf.len() - 1 - i], 0, "back guard {i} clobbered");
        }
    }
    fn pixel(&self, x: u32, y: u32) -> u32 {
        self.buf[GUARD + (y * W + x) as usize]
    }
}

fn opaque_src(w: u32, h: u32, color: u32) -> (Vec<u32>, Surface) {
    let mut pixels = vec![color; (w * h) as usize];
    let base = pixels.as_mut_ptr() as u64;
    let s =
        Surface { base_va: base, stride: w * 4, width: w, height: h, byte_len: (w * h * 4) as u64 };
    (pixels, s)
}

#[test]
fn fill_stays_inside_the_surface_when_rect_overflows() {
    let mut c = Canvas::new();
    let s = c.surface();
    // A rect that runs off the right and bottom edges must be clipped, not wrap.
    fill_rect(s, Rect { x: 30, y: 25, width: 100, height: 100 }, 0xFFFF_0000);
    c.assert_guards_intact();
    assert_eq!(c.pixel(39, 29), 0xFFFF_0000, "bottom-right corner should be filled");
    assert_eq!(c.pixel(0, 0), 0, "far corner should be untouched");
}

#[test]
fn composite_clips_to_both_surface_and_clip_rect() {
    let mut c = Canvas::new();
    let s = c.surface();
    let (_keep, src) = opaque_src(20, 20, 0xFF00_FF00);
    // Place the layer straddling the edge, and clip to a small window.
    composite_layer(s, src, 30, 20, 20, 20, Rect { x: 32, y: 22, width: 4, height: 4 });
    c.assert_guards_intact();
    // Only the clipped 4x4 window should have been written.
    assert_eq!(c.pixel(32, 22), 0xFF00_FF00);
    assert_eq!(c.pixel(35, 25), 0xFF00_FF00);
    assert_eq!(c.pixel(36, 26), 0, "outside the clip must stay untouched");
    assert_eq!(c.pixel(31, 21), 0, "outside the clip must stay untouched");
}

#[test]
fn composite_skips_transparent_source_pixels() {
    let mut c = Canvas::new();
    let s = c.surface();
    fill_rect(s, Rect { x: 0, y: 0, width: W, height: H }, 0xFF20_2020);
    let (_keep, src) = opaque_src(8, 8, 0x0000_0000); // alpha 0 everywhere
    composite_layer(s, src, 4, 4, 8, 8, Rect { x: 0, y: 0, width: W, height: H });
    c.assert_guards_intact();
    assert_eq!(c.pixel(6, 6), 0xFF20_2020, "transparent source must not overwrite the background");
}

#[test]
fn composite_off_screen_placement_writes_nothing() {
    let mut c = Canvas::new();
    let s = c.surface();
    let (_keep, src) = opaque_src(10, 10, 0xFFFF_FFFF);
    // Entirely past the right edge.
    composite_layer(s, src, 200, 5, 10, 10, Rect { x: 0, y: 0, width: W, height: H });
    c.assert_guards_intact();
    for y in 0..H {
        for x in 0..W {
            assert_eq!(c.pixel(x, y), 0, "nothing should have been drawn");
        }
    }
}

#[test]
fn composite_blends_a_translucent_source_over_the_destination() {
    let mut c = Canvas::new();
    let s = c.surface();
    // Destination is solid black; source is 50% white. The result should be a
    // mid grey, proving the layer is blended rather than overwritten.
    fill_rect(s, Rect { x: 0, y: 0, width: W, height: H }, 0xFF00_0000);
    let (_keep, src) = opaque_src(4, 4, 0x80FF_FFFF); // alpha 0x80, white
    composite_layer(s, src, 2, 2, 4, 4, Rect { x: 0, y: 0, width: W, height: H });
    c.assert_guards_intact();
    let p = c.pixel(3, 3);
    let ch = |sh: u32| (p >> sh) & 0xFF;
    assert_eq!(p >> 24, 0xFF, "result stays opaque");
    // 0xFF * 128 / 255 ~= 128; allow a rounding margin.
    for sh in [0u32, 8, 16] {
        assert!((120..=136).contains(&ch(sh)), "channel {sh} not blended: {:#010x}", p);
    }
}

#[test]
fn fill_fully_off_screen_is_a_noop() {
    let mut c = Canvas::new();
    let s = c.surface();
    fill_rect(s, Rect { x: W + 5, y: 0, width: 4, height: 4 }, 0xFFAA_AAAA);
    c.assert_guards_intact();
    assert_eq!(c.pixel(0, 0), 0);
}
