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

//! Ask the installer which capsule-store apps are installed. A missing
//! installer, a timeout or a malformed reply all come back as an empty list, so
//! the desktop is never blocked or panicked by the store.

use alloc::vec;
use alloc::vec::Vec;

use super::admissible::admissible;
use super::call::call;
use super::constants::{OP_LIST_INSTALLED, REPLY_CAP};
use super::decode;

pub fn list_installed() -> Vec<Vec<u8>> {
    let mut rx = vec![0u8; REPLY_CAP];
    let Some(total) = call(OP_LIST_INSTALLED, &mut rx) else {
        return Vec::new();
    };
    decode::names(&rx, total).into_iter().filter(|name| admissible(name)).collect()
}
