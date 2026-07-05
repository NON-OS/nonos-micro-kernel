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

use super::filetype::{color, kind_of};
use super::fmt_time::fmt_time;
use super::human_size::human_size;
use super::layout::{FIRST_ROW_Y, LIST_VISIBLE, ROW_HEIGHT};
use super::manifest::{HEIGHT, WIDTH};
use super::preview_paint::paint_preview;
use super::selection::is_selected;
use super::state::{Mode, State};
use super::theme::{BACKGROUND, FOREGROUND, MUTED, SELECTED};

const TEXT_LEFT: u32 = 16;
const GLYPH_W: u32 = 9;
// Right edge of the name column before the size and date columns begin.
const NAME_MAX: usize = 30;
// Right edges of the size and date columns.
const SIZE_END: u32 = 362;
const DATE_END: u32 = WIDTH - TEXT_LEFT;
const STATUS_Y: u32 = HEIGHT - 22;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    if matches!(state.mode, Mode::Help) {
        super::help::paint_help(fb);
        return;
    }
    if let (Mode::Preview, Some(preview)) = (state.mode, state.preview.as_ref()) {
        paint_preview(preview, fb);
        return;
    }
    fb.clear(BACKGROUND);
    fb.text(TEXT_LEFT, 18, b"file_manager", FOREGROUND);
    fb.text(TEXT_LEFT, 38, clip(state.prefix.as_bytes(), 40), MUTED);
    header(state, fb);
    rows(state, fb);
    footer(state, fb);
}

// Sort mode and active filter, right-aligned on the path line.
fn header(state: &State, fb: &mut PaintBuffer) {
    let mut tag = alloc::string::String::from("sort:");
    tag.push_str(core::str::from_utf8(state.sort_mode.label()).unwrap_or("?"));
    if state.sort_rev {
        tag.push_str(" rev");
    }
    if !state.selected.is_empty() {
        tag.push_str(&alloc::format!("  [{}]", state.selected.len()));
    }
    if !state.filter.is_empty() {
        tag.push_str("  /");
        tag.push_str(&state.filter);
    }
    let x = WIDTH.saturating_sub(TEXT_LEFT + GLYPH_W * tag.len() as u32);
    fb.text(x, 18, tag.as_bytes(), MUTED);
}

fn rows(state: &State, fb: &mut PaintBuffer) {
    if state.entries.is_empty() {
        let msg: &[u8] = if state.filter.is_empty() { b"empty directory" } else { b"no matches" };
        fb.text(TEXT_LEFT, FIRST_ROW_Y, msg, MUTED);
        return;
    }
    let mut y = FIRST_ROW_Y;
    for (i, entry) in state.entries.iter().enumerate().skip(state.scroll).take(LIST_VISIBLE) {
        let selected = i == state.cursor;
        if selected {
            fb.fill_rect(TEXT_LEFT, y - 4, WIDTH - 2 * TEXT_LEFT, 18, 0x222F5AA0);
        }
        // A cyan bar on the far left marks a checked (batch-selected) entry.
        if is_selected(state, &entry.full_path) {
            fb.fill_rect(6, y - 4, 4, 18, SELECTED);
        }
        let name_color = if selected { SELECTED } else { color(kind_of(entry)) };
        fb.text(TEXT_LEFT, y, clip(entry.label.as_bytes(), NAME_MAX), name_color);
        if !entry.writable {
            // Read-only marker in the gap between the name and size columns.
            fb.text(288, y, b"ro", 0xFFE0B060);
        }
        if let Some(size) = entry.size {
            // Files show a byte size; a directory shows its item count.
            let text = if entry.is_dir { alloc::format!("{} it", size) } else { human_size(size) };
            right(fb, SIZE_END, y, text.as_bytes(), MUTED);
        }
        if entry.mtime != 0 {
            right(fb, DATE_END, y, fmt_time(entry.mtime).as_bytes(), MUTED);
        }
        y += ROW_HEIGHT;
    }
}

// Status line, plus the prompt or filter caret when one is open.
fn footer(state: &State, fb: &mut PaintBuffer) {
    let count = alloc::format!("{}/{}", state.cursor + 1, state.entries.len().max(1));
    right(fb, DATE_END, 38, count.as_bytes(), MUTED);
    if let Some((files, bytes, max)) = state.usage {
        let cap = alloc::format!("{}/{} files  {} used", files, max, human_size(bytes));
        fb.text(TEXT_LEFT, STATUS_Y - 18, cap.as_bytes(), MUTED);
    }
    fb.text(TEXT_LEFT, STATUS_Y, state.status, MUTED);
    if matches!(state.mode, Mode::Browse) {
        right(fb, DATE_END, STATUS_Y, b"?=help", MUTED);
    }
    match state.mode {
        Mode::Prompt(_) | Mode::Filter => {
            let x = TEXT_LEFT + GLYPH_W * state.status.len() as u32;
            let text = if matches!(state.mode, Mode::Filter) { state.filter.as_bytes() } else { state.input.as_bytes() };
            fb.text(x, STATUS_Y, text, FOREGROUND);
        }
        _ => {}
    }
}

fn right(fb: &mut PaintBuffer, end_x: u32, y: u32, text: &[u8], color: u32) {
    let x = end_x.saturating_sub(GLYPH_W * text.len() as u32);
    fb.text(x, y, text, color);
}

fn clip(bytes: &[u8], max: usize) -> &[u8] {
    &bytes[..bytes.len().min(max)]
}
