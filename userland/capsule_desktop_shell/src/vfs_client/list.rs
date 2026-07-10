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

//! List the immediate children of `prefix`. The desktop always asks for the
//! root, b"/".

use alloc::vec;
use alloc::vec::Vec;

use super::call::call;
use super::constants::OP_LIST;
use super::entry::Entry;
use super::owner_body::owner_body;
use super::parse;

/// Reply buffer size. Comfortably holds the root listing; anything past it is
/// simply not shown rather than risking a large per-tick allocation.
const REPLY_CAP: usize = 16384;

pub fn list(prefix: &[u8]) -> Vec<Entry> {
    let body = owner_body(prefix);
    let mut rx = vec![0u8; REPLY_CAP];
    let Some(total) = call(OP_LIST, &body, &mut rx) else {
        return Vec::new();
    };
    let prefix_str = core::str::from_utf8(prefix).unwrap_or("/");
    parse::children(prefix_str, &rx, total)
}
