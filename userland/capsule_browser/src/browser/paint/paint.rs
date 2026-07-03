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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::paint::{box_page, chrome, document, home_page};
use crate::browser::state::{State, View};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    chrome::paint(state, fb);
    match state.view {
        View::Home => home_page::paint(state, fb),
        View::Page => match state.box_doc.as_ref() {
            Some(doc) => box_page::paint(state, doc, fb),
            None => document::paint(state, fb),
        },
    }
}
