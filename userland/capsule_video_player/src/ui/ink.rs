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


//! The single measured text path: one face selector shared by every painter and
//! every hit-test, so what is drawn and what is measured can never disagree.

use nonos_app_skeleton::paint::PaintBuffer;
use nonos_toolkit::font::ttf::{
    builtin_face, draw_text_tracked, line_height_with, measure_tracked, measure_with, FontRef,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Body,
    Bold,
    Mono,
}

pub const TRACK: f32 = 1.6;

fn face_of(face: Face) -> Option<&'static FontRef<'static>> {
    match face {
        Face::Body => builtin_face(false, false),
        Face::Bold => builtin_face(false, true),
        Face::Mono => builtin_face(true, false),
    }
}

pub fn draw(fb: &mut PaintBuffer, x: u32, y: u32, s: &str, argb: u32, px: f32, face: Face) -> i32 {
    draw_spaced(fb, x, y, s, argb, px, face, 0.0)
}

pub fn draw_spaced(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    s: &str,
    argb: u32,
    px: f32,
    face: Face,
    spacing: f32,
) -> i32 {
    let Some(f) = face_of(face) else {
        return x as i32;
    };
    let (w, h, stride) = (fb.width, fb.height, fb.stride_words as usize);
    draw_text_tracked(f, fb.pixels, stride, w, h, x as i32, y as i32, s, argb, px, spacing)
}

pub fn width(s: &str, px: f32, face: Face) -> u32 {
    width_spaced(s, px, face, 0.0)
}

pub fn width_spaced(s: &str, px: f32, face: Face, spacing: f32) -> u32 {
    let Some(f) = face_of(face) else {
        return 0;
    };
    if spacing == 0.0 {
        measure_with(f, s, px).max(0) as u32
    } else {
        measure_tracked(f, s, px, spacing).max(0) as u32
    }
}

pub fn line_h(px: f32, face: Face) -> u32 {
    face_of(face).map(|f| line_height_with(f, px).max(0) as u32).unwrap_or(0)
}
