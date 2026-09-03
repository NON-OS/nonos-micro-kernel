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

use crate::about::data::{abi, build};
use crate::about::format::trimmed;

use super::super::card::{self, titled};
use super::super::kv::{kv, ROW_H};
use super::super::metrics::CARD_PAD;

pub const BUILD_H: u32 = card::OVERHEAD + ROW_H * 5;

// What this image is, in the order a reader checks it: which release, which
// commit of it, and what it was built by and for.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, w: u32) {
    let top = titled(fb, x, y, w, BUILD_H, b"Build");
    let rows: [(&[u8], &[u8], bool); 5] = [
        (b"Version", trimmed(build::VERSION), true),
        (b"Commit", build::GIT_SHA, true),
        (b"Toolchain", build::TOOLCHAIN, false),
        (b"Architecture", build::ARCH, false),
        (b"ABI", abi::NAME, false),
    ];
    for (i, (label, value, num)) in rows.into_iter().enumerate() {
        let row_y = top + (i as u32 * ROW_H) as i32;
        kv(fb, x + CARD_PAD, row_y, card::inner(w), label, value, num);
    }
}
