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

use crate::protocol::MAX_BINDINGS;

#[derive(Clone, Copy, Default)]
pub struct MscBinding {
    pub interface: u8,
    pub bulk_in: u8,
    pub bulk_out: u8,
    pub max_packet_in: u16,
    pub max_packet_out: u16,
}

pub struct ProbeResult {
    pub bindings: [MscBinding; MAX_BINDINGS],
    pub count: usize,
}

impl ProbeResult {
    pub fn empty() -> Self {
        Self { bindings: [MscBinding::default(); MAX_BINDINGS], count: 0 }
    }
}
