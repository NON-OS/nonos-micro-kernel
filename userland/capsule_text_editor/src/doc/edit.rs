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

use crate::doc::block::{Block, Run};
use crate::doc::style::RunStyle;

impl Block {
    pub fn insert(&mut self, off: usize, text: &str, _style: RunStyle) {
        let off = off.min(self.text.len());
        let bytes = text.as_bytes();

        self.text.splice(off..off, bytes.iter().copied());

        let mut at = 0usize;
        for r in &mut self.runs {
            at += r.len;
            if off <= at {
                r.len += bytes.len();
                break;
            }
        }
    }

    pub fn delete(&mut self, off: usize, n: usize) {
        let off = off.min(self.text.len());
        let n = n.min(self.text.len() - off);

        if n == 0 {
            return;
        }

        let end = off + n;
        self.text.drain(off..end);

        let mut at = 0usize;
        self.runs.retain_mut(|r| {
            let r_end = at + r.len;
            if r_end <= off {
                at = r_end;
                return true;
            }
            if at >= end {
                return true;
            }
            let trim_start = off.saturating_sub(at);
            let trim_end = end.min(r_end) - at;
            r.len -= trim_end - trim_start;
            at = end.min(r_end);
            r.len > 0
        });

        if self.runs.is_empty() {
            self.runs.push(Run { len: 0, style: RunStyle::body() });
        }
    }
}
