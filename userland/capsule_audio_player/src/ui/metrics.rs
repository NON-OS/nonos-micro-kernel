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

//! The section-08 type scale and the space and radius ramps. Every size here
//! already clears the 17 px readable floor the toolkit clamps draw and measure
//! to, so a run measured at one of these sizes is the run that gets painted.

pub const LABEL: f32 = 17.0;
pub const SECONDARY: f32 = 17.0;
pub const DATA: f32 = 18.0;
pub const ITEM: f32 = 19.0;
pub const BODY: f32 = 19.0;
pub const SECTION: f32 = 24.0;
pub const HERO: f32 = 34.0;
pub const PAGE: f32 = 40.0;
pub const DISPLAY: f32 = 52.0;

pub const S1: i32 = 4;
pub const S2: i32 = 6;
pub const S3: i32 = 9;
pub const S4: i32 = 14;
pub const S5: i32 = 18;
pub const S6: i32 = 22;
pub const S7: i32 = 28;
pub const S8: i32 = 38;

pub const R_WIN: u32 = 20;
pub const R_CARD: u32 = 14;
pub const R_COVER: u32 = 10;
pub const R_CTL: u32 = 9;
pub const R_TILE: u32 = 12;
pub const R_THUMB: u32 = 7;

pub fn line_h(px: f32) -> i32 {
    (px * 1.45) as i32
}

pub fn cap_h(px: f32) -> i32 {
    (px * 0.72) as i32
}

pub fn pill(h: i32) -> u32 {
    (h.max(0) / 2) as u32
}
