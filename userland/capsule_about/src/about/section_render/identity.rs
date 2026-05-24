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

use super::row;
use crate::about::data::{abi, build, product};
use crate::about::state::VISIBLE_BODY_LINES;

pub const LINE_COUNT: u32 = 9;

pub fn render(scroll: u32, top: u32, fb: &mut PaintBuffer) {
    let rows: [(&[u8], &[u8]); 9] = [
        (b"Product", product::NAME),
        (b"Tagline", product::TAGLINE),
        (b"Homepage", product::HOMEPAGE),
        (b"Copyright", product::COPYRIGHT),
        (b"Version", build::VERSION),
        (b"Commit", build::GIT_SHA),
        (b"Built (unix)", build::BUILD_UNIX),
        (b"Toolchain", build::TOOLCHAIN),
        (b"ABI", abi::NAME),
    ];
    let end = scroll.saturating_add(VISIBLE_BODY_LINES).min(LINE_COUNT);
    let visible_count = end.saturating_sub(scroll);
    for offset in 0..visible_count {
        let (label, value) = rows[(scroll + offset) as usize];
        row::pair(label, value, row::line_y(offset, top), fb);
    }
}
