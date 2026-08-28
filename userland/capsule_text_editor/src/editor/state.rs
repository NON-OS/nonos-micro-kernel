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

// 256 KiB per document, heap-backed, enough for any real source file while
// keeping a dozen open tabs cheap.
pub const CAPACITY: usize = 256 * 1024;
pub const PATH: &[u8] = b"/notes.txt";

#[derive(Clone, Copy, PartialEq)]
pub enum PromptOp {
    Open,
    Save,
}

pub struct State {
    pub owner_pid: u32,
    // Document bytes, always CAPACITY long; `len` is the live prefix. A Vec so
    // the buffer lives on the heap: a State moves in and out of the tab list,
    // and a fixed array this large would be copied across the stack each time.
    pub buf: alloc::vec::Vec<u8>,
    pub len: usize,
    // Byte offset of the insertion point within `buf`, always on a UTF-8
    // boundary and in 0..=len. Editing and navigation act here, not at the end.
    pub caret: usize,
    // The fixed end of a selection; the caret is the moving end. None means no
    // selection. `selecting` is true while a click-drag is in progress.
    pub sel_anchor: Option<usize>,
    pub selecting: bool,
    pub scroll_line: u32,
    pub visible_rows: u32,
    pub wrap_cols: u32,
    // The code pane's rectangle inside the window, set by the shell each frame
    // so painting and click mapping share the same origin as the chrome.
    pub pane_x: u32,
    pub pane_y: u32,
    pub pane_w: u32,
    pub pane_h: u32,
    // Monospace cell width in pixels, measured from the font each paint so the
    // caret, click mapping, and glyph layout all agree.
    pub glyph_advance: u32,
    pub status: &'static [u8],
    pub path: [u8; 256],
    pub path_len: usize,
    pub prompt: Option<PromptOp>,
    // The Open and Save As prompts edit this, not `path`. They used to type
    // straight into `path`, so cancelling left the document pointing at
    // whatever had been half typed, and the next save wrote that instead.
    pub prompt_path: [u8; 256],
    pub prompt_len: usize,
    pub shell_port: u32,
    // Undo and redo stacks of reversible edits. Every mutation goes through
    // `apply_edit`, so both stay in sync with the buffer.
    pub undo: alloc::vec::Vec<super::edit::EditOp>,
    pub redo: alloc::vec::Vec<super::edit::EditOp>,
    // Incremental find: the query being typed and whether the find bar is open.
    pub find_active: bool,
    pub find_buf: alloc::string::String,
    // Replace mode layered on find: typing edits the replacement text, Enter
    // rewrites the current match and steps to the next one.
    pub replace_active: bool,
    pub replace_buf: alloc::string::String,
    // The previous press, for double-click detection: when the next press lands
    // close to this in both time and space it selects the word under it.
    pub last_click_ns: u64,
    pub last_click_x: i32,
    pub last_click_y: i32,
    // Zoom level (1..=6) driving the body font size and line height. Default 2
    // reproduces the original 15px look exactly. The theme is process-wide, so
    // it does not live here.
    pub font_scale: u32,
    // The styled document model, rebuilt from `buf` on every mutation while the
    // editor is in Document mode, plus the pages it lays out into. Painting and
    // hit-testing both read `pages`, so a click lands where the glyph was drawn.
    pub doc: crate::doc::document::Doc,
    pub pages: alloc::vec::Vec<crate::doc::page::Page>,
    pub page_metrics: crate::doc::page::PageMetrics,
    pub mode: super::mode::Mode,
}
