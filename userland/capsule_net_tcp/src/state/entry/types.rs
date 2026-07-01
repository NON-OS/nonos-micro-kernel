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

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::tcp::Tcb;

pub const RX_DEPTH: usize = 32;

pub struct Entry {
    pub owner_pid: u32,
    pub handle: u32,
    pub parent: u32,
    pub tcb: Tcb,
    pub rx: VecDeque<Vec<u8>>,
    pub accept: VecDeque<u32>,
    pub snd_buf: VecDeque<u8>,
    pub retx: crate::state::RetxQueue,
    pub rtt: crate::tcp::rtt::Rtt,
    pub reasm: crate::state::Reasm,
    pub cc: crate::tcp::cc::Cc,
}
