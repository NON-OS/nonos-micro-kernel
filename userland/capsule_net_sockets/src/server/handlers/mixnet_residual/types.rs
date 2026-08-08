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

use spin::Mutex;

use crate::server::handlers::mixnet_frame::MAX_BODY;
use crate::sockets::SocketKey;

/// How many sockets may be holding an unread remainder at once. A slot is
/// only taken while a reader is behind on one frame, so this sits well above
/// what a client with a normal read size ever uses.
pub const SLOTS: usize = 16;

pub struct Slot {
    pub pid: u32,
    pub handle: u32,
    pub buf: [u8; MAX_BODY],
    pub len: usize,
    pub off: usize,
}

impl Slot {
    pub const fn empty() -> Self {
        Self { pid: 0, handle: 0, buf: [0u8; MAX_BODY], len: 0, off: 0 }
    }

    pub fn holds(&self, key: SocketKey) -> bool {
        self.off < self.len && self.pid == key.pid && self.handle == key.handle
    }
}

pub static RESIDUAL: Mutex<[Slot; SLOTS]> = Mutex::new([const { Slot::empty() }; SLOTS]);
