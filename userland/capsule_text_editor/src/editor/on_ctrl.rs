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

use nonos_app_skeleton::EventOutcome;

use super::ctrl_copy::ctrl_copy;
use super::ctrl_cut::ctrl_cut;
use super::ctrl_paste::ctrl_paste;
use super::layout::{MAX_SCALE, MIN_SCALE};
use super::path_prompt;
use super::state::{PromptOp, State};
use super::theme;

pub(super) fn on_ctrl(state: &mut State, code: u32, shift: bool) -> EventOutcome {
    match code {
        0x41 | 0x61 => select_all(state),
        0x43 | 0x63 => ctrl_copy(state),
        0x44 | 0x64 => line_edit(state, false),
        0x46 | 0x66 => open_find(state),
        0x48 | 0x68 if shift => replace_all(state),
        0x48 | 0x68 => open_replace(state),
        0x4B | 0x6B if shift => line_edit(state, true),
        0x45 | 0x65 => path_prompt::start(state, PromptOp::Export),
        0x4F | 0x6F => path_prompt::start(state, PromptOp::Open),
        // Ctrl+S writes straight to the file's own path; Ctrl+Shift+S (and a
        // document that never had a path) goes through the Save As prompt.
        0x53 | 0x73 if shift || state.path_len == 0 => path_prompt::start(state, PromptOp::Save),
        0x53 | 0x73 => super::ctrl_save::ctrl_save(state),
        0x56 | 0x76 => ctrl_paste(state),
        0x58 | 0x78 => ctrl_cut(state),
        0x2F => comment(state),
        // Ctrl+Z undoes; Ctrl+Y or Ctrl+Shift+Z redoes.
        0x5A | 0x7A => history(state, !shift),
        0x59 | 0x79 => history(state, false),
        // Ctrl+= / Ctrl++ zoom in, Ctrl+- / Ctrl+_ zoom out, Ctrl+0 reset.
        0x3D | 0x2B => zoom(state, 1),
        0x2D | 0x5F => zoom(state, -1),
        0x30 => zoom_reset(state),
        // Ctrl+B cycles the editor theme.
        0x42 | 0x62 => cycle_theme(state),
        _ => EventOutcome::Idle,
    }
}

// Step the zoom level within bounds; a no-op at the limit stays idle so the
// screen is not repainted for nothing.
fn zoom(state: &mut State, delta: i32) -> EventOutcome {
    let next = (state.font_scale as i32 + delta).clamp(MIN_SCALE as i32, MAX_SCALE as i32) as u32;
    if next == state.font_scale {
        return EventOutcome::Idle;
    }
    state.font_scale = next;
    state.status = b"zoom";
    EventOutcome::Repaint
}

fn zoom_reset(state: &mut State) -> EventOutcome {
    if state.font_scale == 2 {
        return EventOutcome::Idle;
    }
    state.font_scale = 2;
    state.status = b"zoom reset";
    EventOutcome::Repaint
}

fn cycle_theme(state: &mut State) -> EventOutcome {
    theme::cycle();
    state.status = b"theme";
    EventOutcome::Repaint
}

// Ctrl+/ comments or uncomments the caret line or the selected lines.
fn comment(state: &mut State) -> EventOutcome {
    if state.toggle_comment() {
        let rows = state.visible_rows;
        super::follow_caret::follow_caret(state, rows);
        state.status = b"edited";
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

// Ctrl+D duplicates the caret line; Ctrl+Shift+K deletes it. Both follow the
// caret to where it lands and mark the buffer edited.
fn line_edit(state: &mut State, delete: bool) -> EventOutcome {
    let changed = if delete { state.delete_line() } else { state.duplicate_line() };
    if changed {
        let rows = state.visible_rows;
        super::follow_caret::follow_caret(state, rows);
        state.status = b"edited";
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

// Ctrl-Z undoes, Ctrl-Y redoes; both collapse any selection and follow the
// caret to where the change happened.
fn history(state: &mut State, undo: bool) -> EventOutcome {
    let changed = if undo { state.undo() } else { state.redo() };
    if changed {
        state.clear_sel();
        let rows = state.visible_rows;
        super::follow_caret::follow_caret(state, rows);
        state.status = if undo { b"undo" } else { b"redo" };
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

// Ctrl-A selects the whole buffer.
fn select_all(state: &mut State) -> EventOutcome {
    state.sel_anchor = Some(0);
    state.caret = state.len;
    EventOutcome::Repaint
}

// Ctrl-F opens the find bar, seeding it with the current selection.
fn open_find(state: &mut State) -> EventOutcome {
    state.find_active = true;
    if let Some((s, e)) = state.sel_range() {
        state.find_buf = alloc::string::String::from_utf8_lossy(&state.buf[s..e]).into_owned();
    }
    state.find_incremental();
    EventOutcome::Repaint
}

// Ctrl-H opens find with the replacement field focused.
fn open_replace(state: &mut State) -> EventOutcome {
    open_find(state);
    state.replace_active = true;
    EventOutcome::Repaint
}

// Ctrl-Shift-H rewrites every match in one pass and reports the count.
fn replace_all(state: &mut State) -> EventOutcome {
    let n = state.replace_all();
    state.status = if n > 0 { b"replaced all" } else { b"no matches" };
    let rows = state.visible_rows;
    super::follow_caret::follow_caret(state, rows);
    EventOutcome::Repaint
}
