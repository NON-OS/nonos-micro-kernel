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

// Fixtures shared by the Files-backend proofs, which span several test modules.

use crate::store::Store;

pub const PID: u32 = 7;

// The search axis bits, mirrored from `store/fdtable/search.rs` so the tests
// pin the wire-visible values a client has to send.
pub const SEARCH_NAMES: u32 = 1;
pub const SEARCH_CONTENT: u32 = 2;
pub const SEARCH_CASE: u32 = 4;

// Create a file with `data` through the real open/write path.
pub fn put(store: &mut Store, path: &str, data: &[u8]) {
    let fd = store.open(path, PID, true, true, false, true).expect("open create");
    store.write(fd, PID, data).expect("write");
}
