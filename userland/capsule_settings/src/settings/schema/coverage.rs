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

//! Every field the panel knows about must be reachable from some section, or a
//! setting exists that no screen can show. The check runs at compile time, so
//! adding a field without placing it on a screen fails the build.

use crate::settings::section::{SECTIONS, SECTION_COUNT};

use super::all_fields::ALL_FIELDS;
use super::blocks_for::blocks_for;
use super::rows::Row;

const fn placed(id: u32) -> bool {
    let mut s = 0;
    while s < SECTION_COUNT {
        let blocks = blocks_for(SECTIONS[s]);
        let mut b = 0;
        while b < blocks.len() {
            let rows = blocks[b].rows;
            let mut r = 0;
            while r < rows.len() {
                if let Row::Field(f) = rows[r] {
                    if f as u32 == id {
                        return true;
                    }
                }
                r += 1;
            }
            b += 1;
        }
        s += 1;
    }
    false
}

const fn all_placed() -> bool {
    let mut i = 0;
    while i < ALL_FIELDS.len() {
        if !placed(ALL_FIELDS[i] as u32) {
            return false;
        }
        i += 1;
    }
    true
}

const _: () = assert!(all_placed(), "every policy field must appear on a settings screen");
