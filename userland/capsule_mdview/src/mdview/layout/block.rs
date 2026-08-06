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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    H1,
    H2,
    H3,
    Body,
    Bullet,
    Code,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    pub text: String,
    pub mono: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub style: Style,
    pub spans: Vec<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub style: Style,
    pub spans: Vec<Span>,
    pub lead: bool,
}

impl Block {
    pub fn new(style: Style) -> Self {
        Block {
            style,
            spans: Vec::new(),
        }
    }

    pub fn push(&mut self, text: &str, mono: bool) {
        if text.is_empty() {
            return;
        }
        if let Some(last) = self.spans.last_mut() {
            if last.mono == mono {
                last.text.push_str(text);
                return;
            }
        }
        self.spans.push(Span {
            text: String::from(text),
            mono,
        });
    }

    pub fn is_blank(&self) -> bool {
        self.spans.iter().all(|span| span.text.trim().is_empty())
    }
}
