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

use super::types::Scrollback;

impl Scrollback {
    // Begin diverting `push_line` output into a buffer instead of the
    // visible ring, so a command's output can be redirected to a file.
    pub fn begin_capture(&mut self) {
        self.capture = Some(Vec::new());
    }

    // Stop capturing and return the buffered lines. Returns an empty
    // vector if capture was not active.
    pub fn end_capture(&mut self) -> Vec<Vec<u8>> {
        self.capture.take().unwrap_or_default()
    }
}
