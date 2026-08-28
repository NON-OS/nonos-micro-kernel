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

mod para;
mod parts;
mod run;

use alloc::string::String;
use alloc::vec::Vec;

use crate::doc::document::Doc;
use crate::doc::export::zip::Zip;


pub fn document_xml(doc: &Doc) -> String {
    let mut out = String::from(parts::DOCUMENT_HEAD);
    for b in &doc.blocks {
        para::paragraph(b, &mut out);
    }
    out.push_str(parts::DOCUMENT_TAIL);
    out
}

pub fn to_docx(doc: &Doc) -> Vec<u8> {
    let body = document_xml(doc);
    let mut zip = Zip::new();
    zip.add("[Content_Types].xml", parts::CONTENT_TYPES.as_bytes());
    zip.add("_rels/.rels", parts::ROOT_RELS.as_bytes());
    zip.add("word/document.xml", body.as_bytes());
    zip.add("word/_rels/document.xml.rels", parts::DOC_RELS.as_bytes());
    zip.finish()
}
