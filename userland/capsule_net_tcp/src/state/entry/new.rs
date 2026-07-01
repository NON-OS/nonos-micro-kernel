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

use super::types::{Entry, RX_DEPTH};
use crate::tcp::Tcb;

impl Entry {
    pub fn new(owner_pid: u32, handle: u32, parent: u32, tcb: Tcb) -> Self {
        Self {
            owner_pid,
            handle,
            parent,
            tcb,
            rx: VecDeque::with_capacity(RX_DEPTH),
            accept: VecDeque::with_capacity(RX_DEPTH),
            snd_buf: VecDeque::new(),
            retx: crate::state::RetxQueue::new(),
            rtt: crate::tcp::rtt::Rtt::new(),
            reasm: crate::state::Reasm::new(),
            cc: crate::tcp::cc::Cc::new(),
        }
    }
}
