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

use alloc::vec;
use alloc::vec::Vec;

use crate::doc::align::Align;
use crate::doc::kind::BlockKind;
use crate::doc::style::RunStyle;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Run {
    pub len: usize,
    pub style: RunStyle,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub kind: BlockKind,
    pub text: Vec<u8>,
    pub runs: Vec<Run>,
    pub align: Align,
}

impl Block {
    pub fn plain(kind: BlockKind, text: &str, style: RunStyle) -> Self {
        Self {
            kind,
            text: text.as_bytes().to_vec(),
            runs: vec![Run { len: text.len(), style }],
            align: Align::Left,
        }
    }

    pub fn covered(&self) -> bool {
        self.runs.iter().map(|r| r.len).sum::<usize>() == self.text.len()
    }

    pub fn style_at(&self, off: usize) -> RunStyle {
        let mut at = 0usize;
        for r in &self.runs {
            at += r.len;
            if off < at {
                return r.style;
            }
        }
        self.runs.last().map(|r| r.style).unwrap_or_else(RunStyle::body)
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.text).unwrap_or("")
    }
}
