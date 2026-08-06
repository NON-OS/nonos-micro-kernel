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

use super::block::{Block, Line, Span, Style};

pub fn code_lines(block: &Block, out: &mut Vec<Line>) {
    let mut source = String::new();
    for span in &block.spans {
        source.push_str(&span.text);
    }
    let mut lead = true;
    for raw in source.trim_end_matches('\n').split('\n') {
        out.push(Line {
            style: Style::Code,
            spans: vec![Span {
                text: String::from(raw.trim_end()),
                mono: true,
            }],
            lead,
        });
        lead = false;
    }
}
