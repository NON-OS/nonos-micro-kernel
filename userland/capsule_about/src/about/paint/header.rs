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

use nonos_app_skeleton::PaintBuffer;

use crate::about::data::product::{NAME, TAGLINE};
use crate::about::format::u64_decimal;
use crate::about::section::SECTIONS;
use crate::about::state::State;
use crate::about::theme::{HEADER, HEADER_HEIGHT, HEADLINE, TEXT_LEFT};

const BREADCRUMB_RIGHT_PADDING: u32 = 12;
const CELL_WIDTH: u32 = 8;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(0, 0, fb.width, HEADER_HEIGHT, HEADER);
    fb.text(TEXT_LEFT, 12, NAME, HEADLINE);
    fb.text(TEXT_LEFT + 64, 16, TAGLINE, HEADLINE);
    let mut buf_idx = [0u8; 20];
    let mut buf_total = [0u8; 20];
    let idx_text = u64_decimal((state.section.index() + 1) as u64, &mut buf_idx);
    let total_text = u64_decimal(SECTIONS.len() as u64, &mut buf_total);
    let mut crumb = [0u8; 32];
    let mut written = 0usize;
    for &b in idx_text {
        crumb[written] = b;
        written += 1;
    }
    crumb[written] = b' ';
    written += 1;
    crumb[written] = b'/';
    written += 1;
    crumb[written] = b' ';
    written += 1;
    for &b in total_text {
        crumb[written] = b;
        written += 1;
    }
    let crumb_width = (written as u32) * CELL_WIDTH;
    let x = fb.width.saturating_sub(crumb_width + BREADCRUMB_RIGHT_PADDING);
    fb.text(x, 16, &crumb[..written], HEADLINE);
}
