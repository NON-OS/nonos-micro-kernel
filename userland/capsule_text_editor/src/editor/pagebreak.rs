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

//! Insert > Page Break. The break is a form feed alone on its line, so it
//! lives in the text buffer itself and is rebuilt by `doc_from_text` on every
//! reflow the way a heading prefix is, rather than being kept beside it.

use super::app::Editor;
use super::mode::Mode;
use super::unsupported::NO_DOC_MODE;

impl Editor {
    pub(super) fn insert_page_break(&mut self) {
        let doc = self.doc();
        if doc.mode != Mode::Document {
            doc.status = NO_DOC_MODE;
            return;
        }
        let at = doc.caret.min(doc.len);
        let opening = at > 0 && doc.buf.get(at - 1) != Some(&b'\n');
        let bytes: &[u8] = if opening { b"\n\x0c\n" } else { b"\x0c\n" };
        let _ = doc.insert(bytes);
    }
}
