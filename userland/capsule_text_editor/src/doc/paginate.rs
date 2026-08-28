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

use crate::doc::document::Doc;
use crate::doc::linebreak::break_block;
use crate::doc::measure::Measurer;
use crate::doc::page::{Page, PageMetrics};

pub fn paginate(doc: &Doc, pm: &PageMetrics, m: &dyn Measurer) -> Vec<Page> {
    let max_h = pm.content_height();
    let mut pages = Vec::new();
    let mut page = Page::new();
    let mut y = 0.0f32;
    for (i, block) in doc.blocks.iter().enumerate() {
        for mut line in break_block(block, i, pm.content_width(), m) {
            if y + line.height > max_h && !page.lines.is_empty() {
                pages.push(page);
                page = Page::new();
                y = 0.0;
            }
            line.y = y;
            y += line.height;
            page.lines.push(line);
        }
    }
    pages.push(page);
    pages
}
