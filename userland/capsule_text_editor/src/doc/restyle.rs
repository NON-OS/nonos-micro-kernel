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

fn split_at(block: &mut Block, off: usize) {
    let mut at = 0usize;
    for i in 0..block.runs.len() {
        let end = at + block.runs[i].len;
        if off > at && off < end {
            let left_len = off - at;
            let right_len = block.runs[i].len - left_len;
            let style = block.runs[i].style;
            block.runs[i].len = left_len;
            block.runs.insert(i + 1, Run {
                len: right_len,
                style,
            });
            return;
        }
        at = end;
    }
}

fn merge(block: &mut Block) {
    let mut i = 0;
    while i + 1 < block.runs.len() {
        if block.runs[i].style == block.runs[i + 1].style {
            block.runs[i].len += block.runs[i + 1].len;
            block.runs.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

pub fn set_style(block: &mut Block, off: usize, n: usize, f: &dyn Fn(&mut RunStyle)) {
    let end = off + n;
    if off == end {
        return;
    }

    split_at(block, off);
    split_at(block, end);

    let mut at = 0usize;
    for r in &mut block.runs {
        let r_end = at + r.len;
        if at >= off && r_end <= end {
            f(&mut r.style);
        }
        at = r_end;
    }

    merge(block);
}
