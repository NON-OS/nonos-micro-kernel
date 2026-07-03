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

use alloc::vec::Vec;

use crate::browser::html::flow::Flow;
use crate::browser::layout;

use super::render_response::Rendered;

// Line renderer for non-HTML content: plain text and error surfaces.
pub(super) fn render_lines(flows: Vec<Flow>) -> (Rendered, usize) {
    if flows.is_empty() {
        return (Rendered::Nothing, 0);
    }
    let n = flows.len();
    let doc =
        layout::build(&flows, crate::browser::manifest::WIDTH, nonos_app_skeleton::font_advance());
    (Rendered::Lines(doc), n)
}
