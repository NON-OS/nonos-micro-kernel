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
use spin::Mutex;

use crate::state::Entry;
use crate::state::Timers;
use crate::tcp::{iss_for, Endpoint4};

pub const TABLE_CAP: usize = 256;
pub static TABLE: Mutex<Table> = Mutex::new(Table::new());

pub struct Table {
    pub(super) entries: Vec<Entry>,
    pub(super) next_handle: u32,
    pub(crate) timers: Timers,
    pub(super) iss_key: [u64; 2],
}

impl Table {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_handle: 1,
            timers: Timers::new(),
            iss_key: [0, 0],
        }
    }

    pub fn seed_iss(&mut self, key: [u64; 2]) {
        self.iss_key = key;
    }

    pub fn iss_for_pair(&self, local: Endpoint4, remote: Endpoint4) -> u32 {
        iss_for(self.iss_key, local.ip, local.port, remote.ip, remote.port)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TableError {
    Full,
    NotFound,
}
