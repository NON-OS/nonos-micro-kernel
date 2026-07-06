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

// Install a fetched face body under its family key: a WOFF unwraps to sfnt
// first, a raw ttf/otf loads as is. Returns true when the face installed and
// the page should relayout with its real metrics.
pub fn ingest_font(key: u32, body: Vec<u8>) -> bool {
    if body.is_empty() {
        return false;
    }
    let Some(sfnt) = super::woff::unwrap_woff(body) else { return false };
    super::registry::install(key, sfnt)
}
