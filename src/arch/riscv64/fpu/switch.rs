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

use super::current::slot_mut;
use super::enable::{disable, enable_initial, mark_dirty};
use super::restore::restore;
use super::save::save;

pub fn save_outgoing() {
    let slot = match slot_mut() {
        Some(s) => s,
        None => {
            disable();
            return;
        }
    };
    if slot.enabled && slot.dirty {
        unsafe { save(&mut slot.ctx) };
        slot.valid = true;
        slot.dirty = false;
    }
    disable();
    slot.enabled = false;
}

pub fn prepare_incoming() {
    let slot = match slot_mut() {
        Some(s) => s,
        None => {
            disable();
            return;
        }
    };
    if slot.valid {
        enable_initial();

        unsafe { restore(&slot.ctx) };
        mark_dirty();
        slot.enabled = true;
        slot.dirty = false;
    } else {
        disable();
        slot.enabled = false;
    }
}
