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

use crate::descriptors::MscBinding;
use crate::protocol::MAX_BINDINGS;

pub struct State {
    pub(super) bindings: [MscBinding; MAX_BINDINGS],
    pub(super) binding_count: usize,
    pub(super) next_tag: u32,
    pub(super) last_tag: u32,
    pub(super) last_data_len: u32,
    pub(super) pending: bool,
    pub(super) probes: u64,
    pub(super) csw_ok: u64,
    pub(super) csw_failed: u64,
    pub(super) phase_errors: u64,
    pub(super) residue_bytes: u64,
}

impl State {
    pub fn new() -> Self {
        Self {
            bindings: [MscBinding::default(); MAX_BINDINGS],
            binding_count: 0,
            next_tag: 1,
            last_tag: 0,
            last_data_len: 0,
            pending: false,
            probes: 0,
            csw_ok: 0,
            csw_failed: 0,
            phase_errors: 0,
            residue_bytes: 0,
        }
    }
}
