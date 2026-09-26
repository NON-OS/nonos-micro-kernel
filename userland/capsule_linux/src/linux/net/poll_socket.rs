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

//! Asking net.sockets whether one handle is ready.

use super::call::call;
use super::ops::{OP_POLL, POLL_READABLE, POLL_WRITABLE};

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;

pub(super) fn socket_bits(handle: u32) -> u16 {
    let Some((0, out)) = call(OP_POLL, &handle.to_le_bytes(), 1) else {
        return 0;
    };
    let Some(bits) = out.first() else {
        return 0;
    };
    let mut set = 0;
    if bits & POLL_READABLE != 0 {
        set |= POLLIN;
    }
    if bits & POLL_WRITABLE != 0 {
        set |= POLLOUT;
    }
    set
}
