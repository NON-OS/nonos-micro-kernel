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

use crate::ui::layout::{Layout, Rect, EDGE, PAD};

pub const BTN: u32 = 38;
pub const HEX_R: u32 = 25;
pub const GAP: u32 = 12;
pub const VOL_W: u32 = 84;
pub const LANG_W: u32 = 62;
pub const MARK: u32 = 16;

pub struct Transport {
    pub shuffle: Rect,
    pub prev: Rect,
    pub play: Rect,
    pub next: Rect,
    pub repeat: Rect,
    pub volume: Rect,
    pub mute: Rect,
    pub cc: Rect,
    pub lang: Rect,
    pub pip: Rect,
    pub full: Rect,
}

fn slot(x: u32, y: u32, w: u32) -> Rect {
    Rect { x, y, w, h: BTN }
}

pub fn transport(l: &Layout, w: u32) -> Transport {
    let y = l.bar.y + 22;
    let cx = w / 2;
    let hex = HEX_R * 2;
    let step = BTN + GAP;
    let inner = HEX_R + GAP + BTN / 2;
    let outer = inner + step;
    let right = w.saturating_sub(EDGE + PAD);
    let full = right.saturating_sub(BTN);
    let pip = full.saturating_sub(step);
    let lang = pip.saturating_sub(LANG_W + GAP);
    let cc = lang.saturating_sub(step);
    let vol = cc.saturating_sub(VOL_W + GAP);
    Transport {
        shuffle: slot(cx.saturating_sub(outer + BTN / 2), y, BTN),
        prev: slot(cx.saturating_sub(inner + BTN / 2), y, BTN),
        play: slot(cx.saturating_sub(HEX_R), y, hex),
        next: slot(cx + inner.saturating_sub(BTN / 2), y, BTN),
        repeat: slot(cx + outer.saturating_sub(BTN / 2), y, BTN),
        volume: Rect { x: vol, y: y + BTN / 2 - 3, w: VOL_W, h: 6 },
        mute: Rect { x: vol.saturating_sub(26), y: y + BTN / 2 - 9, w: MARK, h: MARK },
        cc: slot(cc, y, BTN),
        lang: Rect { x: lang, y: y + 4, w: LANG_W, h: 30 },
        pip: slot(pip, y, BTN),
        full: slot(full, y, BTN),
    }
}
