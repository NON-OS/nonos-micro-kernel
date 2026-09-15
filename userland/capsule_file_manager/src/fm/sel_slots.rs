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

extern crate alloc;

use alloc::vec::Vec;

use nonos_app_skeleton::measure_ttf;

use super::header_slots::{GLYPH_GAP, GLYPH_S, ICON_BTN, ICON_PAD, TOOL_PX};
use super::icon_path::Icon;
use super::sel_geom::{SelBand, GAP, PAD};
use super::sel_model::{SelAction, ACTIONS};

/// One action's box inside the band.
pub struct SelSlot {
    pub x: u32,
    pub w: u32,
    pub action: SelAction,
    pub icon: Icon,
    pub label: &'static str,
    pub wired: bool,
}

/// A labelled pill's width: the same padding `icon_label` draws with, around the
/// label measured off the free function -- so the hit-test, which has no
/// PaintBuffer, sizes every pill exactly as the painter did.
pub fn labelled_w(label: &str) -> u32 {
    ICON_PAD * 2 + GLYPH_S + GLYPH_GAP + measure_ttf(label, TOOL_PX).max(0) as u32
}

/// Every action's box, laid out left to right in one pass, so a click resolves
/// against exactly the pill that was drawn.
pub fn slots(b: &SelBand) -> Vec<SelSlot> {
    let mut out = Vec::new();
    let mut x = b.x + PAD;
    for (action, icon, label, wired) in ACTIONS {
        let w = if b.labelled { labelled_w(label) } else { ICON_BTN };
        out.push(SelSlot { x, w, action, icon, label, wired });
        x += w + GAP;
    }
    out
}
