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

mod content;
mod fmt;
mod objects;
mod pageobj;
mod text;
mod writer;

use alloc::vec::Vec;

use crate::doc::document::Doc;
use crate::doc::measure::Measurer;
use crate::doc::page::PageMetrics;
use crate::doc::paginate::paginate;

pub fn to_pdf(doc: &Doc, pm: &PageMetrics, m: &dyn Measurer) -> Vec<u8> {
    let pages = paginate(doc, pm, m);
    let mut bodies = Vec::with_capacity(6 + 2 * pages.len());
    bodies.push(objects::catalog());
    bodies.push(objects::pages(pages.len()));
    for name in objects::FONTS.iter() {
        bodies.push(objects::font(name));
    }
    for (i, page) in pages.iter().enumerate() {
        bodies.push(pageobj::page_obj(objects::page_obj_id(i) + 1, pm));
        bodies.push(pageobj::stream_obj(&content::page_content(doc, page, pm)));
    }
    writer::assemble(&bodies)
}
