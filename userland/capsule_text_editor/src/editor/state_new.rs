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

use super::state::{State, PATH};

impl State {
    pub fn new() -> Self {
        let mut path = [0u8; 256];
        path[..PATH.len()].copy_from_slice(PATH);
        State {
            owner_pid: 0,
            buf: alloc::vec![0; super::state::CAPACITY],
            len: 0,
            caret: 0,
            sel_anchor: None,
            selecting: false,
            scroll_line: 0,
            visible_rows: 1,
            wrap_cols: 80,
            pane_x: 0,
            pane_y: 0,
            pane_w: 0,
            pane_h: 0,
            glyph_advance: super::layout::GLYPH_ADVANCE,
            status: b"Ctrl-O open  Ctrl-S save  Ctrl-C copy  Ctrl-V paste",
            path,
            path_len: PATH.len(),
            prompt: None,
            prompt_path: [0u8; 256],
            prompt_len: 0,
            shell_port: 0,
            undo: alloc::vec::Vec::new(),
            redo: alloc::vec::Vec::new(),
            find_active: false,
            find_buf: alloc::string::String::new(),
            replace_active: false,
            replace_buf: alloc::string::String::new(),
            last_click_ns: 0,
            last_click_x: 0,
            last_click_y: 0,
            font_scale: 2,
            doc: crate::doc::document::Doc::new(),
            pages: alloc::vec::Vec::new(),
            page_metrics: crate::doc::page::PageMetrics {
                width: 760.0,
                height: 980.0,
                margin: 56.0,
            },
            mode: super::mode::Mode::Code,
        }
    }
}
