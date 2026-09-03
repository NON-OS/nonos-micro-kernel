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

use crate::about::data::third_party::COMPONENTS;
use crate::about::theme::{FOREGROUND, MUTED, RULE};

use super::super::card::{self, titled};
use super::super::kv::{fit_mono, ROW_H};
use super::super::metrics::{BODY_PX, CARD_PAD, KV_GAP, NUM_PX};
use super::super::text::{self, line, mono, rule, top_of};
use super::licenses_cols::{band, columns};

pub const HEAD_COMPONENT: &[u8] = b"COMPONENT";
pub const HEAD_ROLE: &[u8] = b"ROLE";
const HEAD_LICENSE: &[u8] = b"LICENCE";
const HEAD_RULE_GAP: u32 = 6;

pub const HEIGHT: u32 =
    card::OVERHEAD + ROW_H + HEAD_RULE_GAP + ROW_H * COMPONENTS.len() as u32;

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let top = titled(fb, 0, y, w, HEIGHT, b"Third-party components");
    let (role_x, license_x) = columns();
    let inner = card::inner(w);
    let head = top_of(top, ROW_H, BODY_PX);
    line(fb, CARD_PAD, head, HEAD_COMPONENT, MUTED, BODY_PX);
    line(fb, role_x, head, HEAD_ROLE, MUTED, BODY_PX);
    line(fb, license_x, head, HEAD_LICENSE, MUTED, BODY_PX);
    rule(fb, CARD_PAD, top + (ROW_H + HEAD_RULE_GAP / 2) as i32, inner, RULE);
    let first = top + (ROW_H + HEAD_RULE_GAP) as i32;
    let name_w = role_x.saturating_sub(CARD_PAD + KV_GAP);
    let license_w = (CARD_PAD + inner).saturating_sub(license_x);
    for (i, c) in COMPONENTS.iter().enumerate() {
        let row_y = first + (i as u32 * ROW_H) as i32;
        if i % 2 == 1 {
            band(fb, row_y, inner);
        }
        let cell = top_of(row_y, ROW_H, BODY_PX);
        line(fb, CARD_PAD, cell, text::fit(fb, c.name, BODY_PX, name_w), FOREGROUND, BODY_PX);
        mono(fb, role_x, cell, fit_mono(fb, c.role, license_x - role_x - KV_GAP), MUTED, NUM_PX);
        mono(fb, license_x, cell, fit_mono(fb, c.license, license_w), FOREGROUND, NUM_PX);
    }
}
