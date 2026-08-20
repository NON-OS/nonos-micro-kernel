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

use super::frame_rect::{frame_rect, light_rect, titlebar_rect};
use super::metrics::LIGHT_HIT_PAD;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DecorationHit {
    None,
    Titlebar,
    CloseButton,
    MinimizeButton,
    MaximizeButton,
}

const LIGHT_HITS: [DecorationHit; 3] =
    [DecorationHit::CloseButton, DecorationHit::MinimizeButton, DecorationHit::MaximizeButton];

pub fn hit_test(w: u32, h: u32, maximized: bool, x: u32, y: u32) -> DecorationHit {
    if !frame_rect(w, h, maximized).contains(x, y) {
        return DecorationHit::None;
    }
    if !titlebar_rect(w, h, maximized).contains(x, y) {
        return DecorationHit::None;
    }
    for (i, hit) in LIGHT_HITS.iter().enumerate() {
        if light_rect(i as u32, w, h, maximized).inflate(LIGHT_HIT_PAD).contains(x, y) {
            return *hit;
        }
    }
    DecorationHit::Titlebar
}
