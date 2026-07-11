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

//! Ctrl+/ toggles the line comment over the caret line or the selected lines,
//! using the marker for the file's language. If every non-blank line is already
//! commented the comment is removed, otherwise it is added at each line's first
//! non-blank column. The whole region changes in one undoable edit.

use alloc::vec::Vec;

use super::language::comment_prefix;
use super::line_bounds::{line_end, line_start};
use super::state::State;

impl State {
    pub fn toggle_comment(&mut self) -> bool {
        let path = core::str::from_utf8(&self.path[..self.path_len]).unwrap_or("");
        let prefix = comment_prefix(path);

        // Whole lines: from the start of the first selected line to the end of
        // the last, so a caret anywhere on a line comments all of it.
        let (s, e) = self.sel_range().unwrap_or((self.caret, self.caret));
        let start = line_start(&self.buf[..self.len], s);
        let end = line_end(&self.buf[..self.len], e).max(start);
        let region: Vec<u8> = self.buf[start..end].to_vec();

        let lines: Vec<&[u8]> = region.split(|&b| b == b'\n').collect();
        let all_commented = lines.iter().filter(|l| !is_blank(l)).all(|l| commented(l, prefix));

        let mut out = Vec::with_capacity(region.len() + lines.len() * (prefix.len() + 1));
        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                out.push(b'\n');
            }
            if is_blank(line) {
                out.extend_from_slice(line);
                continue;
            }
            let indent = leading_ws(line);
            out.extend_from_slice(&line[..indent]);
            let body = &line[indent..];
            if all_commented {
                out.extend_from_slice(uncomment(body, prefix));
            } else {
                out.extend_from_slice(prefix);
                out.push(b' ');
                out.extend_from_slice(body);
            }
        }

        let new_len = out.len();
        if !self.apply_edit(start, end - start, &out) {
            return false;
        }
        self.sel_anchor = Some(start);
        self.caret = start + new_len;
        true
    }
}

fn leading_ws(line: &[u8]) -> usize {
    line.iter().take_while(|&&b| b == b' ' || b == b'\t').count()
}

fn is_blank(line: &[u8]) -> bool {
    leading_ws(line) == line.len()
}

fn commented(line: &[u8], prefix: &[u8]) -> bool {
    line[leading_ws(line)..].starts_with(prefix)
}

// Drop the leading comment marker and one following space, if present.
fn uncomment<'a>(body: &'a [u8], prefix: &[u8]) -> &'a [u8] {
    let rest = &body[prefix.len()..];
    if rest.first() == Some(&b' ') {
        &rest[1..]
    } else {
        rest
    }
}
