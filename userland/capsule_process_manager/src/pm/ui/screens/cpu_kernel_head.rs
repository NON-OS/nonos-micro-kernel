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

//! The two lines above the kernel traces: the load averages and how many
//! processors the kernel brought online.

use nonos_app_skeleton::PaintBuffer;

use crate::pm::format::u32_decimal;
use crate::pm::format_sys::load_human;
use crate::pm::state::State;
use crate::pm::theme::{MUTED, TITLE};

use super::super::metrics::{BAR_ROW_H, BODY_PX};
use super::super::text;

pub(super) fn head(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, right: u32) -> u32 {
    let mut buf = [0u8; 40];
    let mut n = 0;
    for (i, q11) in state.sys.load.iter().enumerate() {
        if i > 0 {
            buf[n..n + 3].copy_from_slice(b" / ");
            n += 3;
        }
        n += load_human(*q11, &mut buf[n..]);
    }
    let y = line(fb, x, y, right, b"load 1 / 5 / 15 min", &buf[..n]);
    let n = u32_decimal(state.sys.cpus, &mut buf);
    line(fb, x, y, right, b"processors online", &buf[..n])
}

fn line(fb: &mut PaintBuffer, x: u32, y: u32, right: u32, label: &[u8], value: &[u8]) -> u32 {
    let top = text::centred_top(y, BAR_ROW_H, BODY_PX);
    text::left(fb, x, top, label, MUTED, BODY_PX);
    text::mono_right(fb, right, top, value, TITLE, BODY_PX);
    y + BAR_ROW_H
}
