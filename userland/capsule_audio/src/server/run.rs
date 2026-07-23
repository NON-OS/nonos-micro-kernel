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

use alloc::vec;

use nonos_libc::mk_ipc_recv;

use crate::mark::mark;

const RX_LEN: usize = 4096;
const RECV_TIMEOUT_MS: u64 = 1000;

pub fn run() -> ! {
    mark("[AUDIO] up\n");
    let mut rx = vec![0u8; RX_LEN];
    loop {
        let _ = mk_ipc_recv(0, rx.as_mut_ptr(), RX_LEN, RECV_TIMEOUT_MS);
    }
}
