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

use crate::about::data::caps::{is_granted, ALL_CAPS};
use crate::about::theme::{FOREGROUND, MUTED};

use super::super::card::{self, titled};
use super::super::chip_wrap::wrap;
use super::super::kv::ROW_H;
use super::super::metrics::{BODY_PX, CARD_PAD};
use super::super::text::{line, top_of};
use super::trust_caps_list::{height, names, summary, CAP_SLOTS, DENIED_GAP, ROLE_GAP};

// Granted and denied are the same twenty-one bits shown twice: as pills that can
// be counted at a glance, and, for the ones this capsule holds, as the sentence
// that says what each of them actually permits.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let inner = card::inner(w);
    let top = titled(fb, 0, y, w, height(inner), b"Capabilities");
    let mut head = [0u8; 48];
    line(fb, CARD_PAD, top_of(top, ROW_H, BODY_PX), summary(&mut head, true, true), MUTED, BODY_PX);
    let mut slots: [&'static [u8]; CAP_SLOTS] = [b""; CAP_SLOTS];
    let granted = names(&mut slots, true);
    let chips_y = top + ROW_H as i32;
    let chips_h = wrap(fb, CARD_PAD, chips_y, inner, &slots[..granted], true);
    let roles_y = chips_y + (chips_h + ROLE_GAP) as i32;
    roles(fb, roles_y, inner);
    let denied_y = roles_y + (granted as u32 * ROW_H + DENIED_GAP) as i32;
    let mut foot = [0u8; 48];
    let label = summary(&mut foot, false, false);
    line(fb, CARD_PAD, top_of(denied_y, ROW_H, BODY_PX), label, MUTED, BODY_PX);
    let denied = names(&mut slots, false);
    wrap(fb, CARD_PAD, denied_y + ROW_H as i32, inner, &slots[..denied], false);
}

// Name then role on one baseline, drawn as two runs from the measured advance of
// the first so the separator sits against the name whatever the name measures.
fn roles(fb: &mut PaintBuffer, y: i32, inner: u32) {
    let mut row = 0u32;
    for cap in ALL_CAPS {
        if !is_granted(cap.bit) {
            continue;
        }
        let ry = top_of(y + (row * ROW_H) as i32, ROW_H, BODY_PX);
        let x = line(fb, CARD_PAD, ry, cap.name, FOREGROUND, BODY_PX);
        let x = line(fb, x.max(0) as u32, ry, b"   ", MUTED, BODY_PX);
        let left = (CARD_PAD + inner).saturating_sub(x.max(0) as u32);
        let cut = super::super::text::fit(fb, cap.role, BODY_PX, left);
        line(fb, x.max(0) as u32, ry, cut, MUTED, BODY_PX);
        row += 1;
    }
}
