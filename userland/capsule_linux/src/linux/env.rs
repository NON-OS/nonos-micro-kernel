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

//! The environment the first program of a guest starts with.

use alloc::vec::Vec;

/// Deliberately short.
pub fn default() -> Vec<Vec<u8>> {
    alloc::vec![
        b"PATH=/usr/local/bin:/usr/bin:/bin:/usr/local/sbin:/usr/sbin:/sbin".to_vec(),
        b"HOME=/root".to_vec(),
        b"TERM=linux".to_vec(),
        b"PWD=/".to_vec(),
        b"SHELL=/bin/sh".to_vec(),
        b"LANG=C.UTF-8".to_vec(),
    ]
}
