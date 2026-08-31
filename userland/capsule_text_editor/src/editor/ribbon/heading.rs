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

//! The block-style pill's edit. A heading lives in the text as a `#` prefix, so
//! changing one rewrites that prefix through the single undoable edit path and
//! survives every later reflow, unlike a run style.

use alloc::vec::Vec;

use crate::editor::state::State;

fn line_start(buf: &[u8], at: usize) -> usize {
    let mut i = at.min(buf.len());
    while i > 0 && buf[i - 1] != b'\n' {
        i -= 1;
    }
    i
}

fn line_end(buf: &[u8], at: usize) -> usize {
    let mut i = at.min(buf.len());
    while i < buf.len() && buf[i] != b'\n' {
        i += 1;
    }
    i
}

fn prefix_len(line: &[u8]) -> usize {
    let n = line.iter().take_while(|c| **c == b'#').count();
    if (1..=6).contains(&n) && line.get(n) == Some(&b' ') {
        n + 1
    } else {
        0
    }
}

impl State {
    pub(in crate::editor) fn set_heading(&mut self, level: u8) {
        let start = line_start(&self.buf[..self.len], self.caret);
        let end = line_end(&self.buf[..self.len], self.caret);
        let old = prefix_len(&self.buf[start..end]);
        let mut ins: Vec<u8> = Vec::new();
        for _ in 0..level.min(6) {
            ins.push(b'#');
        }
        if level > 0 {
            ins.push(b' ');
        }
        if old == ins.len() && self.buf[start..start + old] == ins[..] {
            return;
        }
        self.sel_anchor = None;
        self.apply_edit(start, old, &ins);
    }
}
