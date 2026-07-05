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

use super::preview::Preview;
use super::theme::{BACKGROUND, DIRECTORY, FOREGROUND, MUTED};

const LEFT: u32 = 16;
const FIRST_Y: u32 = 72;
const LINE_H: u32 = 14;
const MAX_COLS: usize = 49;
pub const VISIBLE_LINES: usize = 17;

// Render an opened file: title, path, a byte/line info bar, then the visible
// window of content lines starting at the preview's scroll offset.
pub fn paint_preview(preview: &Preview, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    fb.text(LEFT, 18, b"file_manager", FOREGROUND);
    fb.text(LEFT, 40, clip(preview.path.as_bytes(), MAX_COLS), DIRECTORY);
    fb.text(LEFT, 56, info(preview).as_bytes(), MUTED);
    let mut y = FIRST_Y;
    for (idx, line) in preview.lines.iter().enumerate().skip(preview.scroll).take(VISIBLE_LINES) {
        if preview.binary {
            // The hexdump already carries byte offsets, so no line gutter.
            fb.text(LEFT, y, clip(line.as_bytes(), MAX_COLS), FOREGROUND);
        } else {
            let num = alloc::format!("{:>4}", idx + 1);
            fb.text(LEFT, y, num.as_bytes(), MUTED);
            fb.text(LEFT + 46, y, clip(line.as_bytes(), MAX_COLS - 6), FOREGROUND);
        }
        y += LINE_H;
    }
    fb.text(LEFT, super::manifest::HEIGHT - 22, b"esc back  up/down scroll", MUTED);
}

// A one-line summary: byte count, binary/truncated flags, and the visible line
// window so the reader knows where they are in a long file.
fn info(preview: &Preview) -> alloc::string::String {
    let total = preview.lines.len();
    let first = if total == 0 { 0 } else { preview.scroll + 1 };
    let last = (preview.scroll + VISIBLE_LINES).min(total);
    let kind = if preview.binary { "binary" } else { "text" };
    let cut = if preview.truncated { " (truncated)" } else { "" };
    alloc::format!("{} bytes  {}  ln {}-{}/{}{}", preview.byte_len, kind, first, last, total, cut)
}

fn clip(bytes: &[u8], max: usize) -> &[u8] {
    &bytes[..bytes.len().min(max)]
}
