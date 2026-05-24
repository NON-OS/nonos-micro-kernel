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

use super::super::gateway::Gateway;
use super::super::session::Session;

pub const TABLE_CAP: usize = 32;
pub static TABLE: Mutex<Table> = Mutex::new(Table::new());

pub struct Table {
    pub(super) gateway: Option<Gateway>,
    pub(super) next_id: u32,
    pub(super) sessions: Vec<Session>,
    pub(super) stream_rx: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableError {
    NoGateway,
    NoTopology,
    StaleTopology,
    NoCredential,
    Full,
}

impl Table {
    pub const fn new() -> Self {
        Self { gateway: None, next_id: 1, sessions: Vec::new(), stream_rx: Vec::new() }
    }
}
