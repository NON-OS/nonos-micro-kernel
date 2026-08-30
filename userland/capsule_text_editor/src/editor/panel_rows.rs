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

//! The row labels behind an open panel, built once and read by both the
//! painter and the hit-test so the two never disagree about the row count.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::app::Editor;
use super::panel::{special_labels, Panel};
use super::wordcount::count_rows;

impl Editor {
    pub(super) fn panel_rows(&self, panel: Panel) -> Vec<String> {
        match panel {
            Panel::WordCount => {
                let i = self.active.min(self.docs.len().saturating_sub(1));
                count_rows(&self.docs[i])
            }
            Panel::Special => special_labels().iter().map(|s| s.to_string()).collect(),
        }
    }
}
