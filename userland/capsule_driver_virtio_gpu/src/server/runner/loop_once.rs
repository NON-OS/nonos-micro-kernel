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

use nonos_libc::mk_ipc_recv_from;

use super::dispatch::dispatch;
use crate::debug;
use crate::driver::Driver;
use crate::protocol::parse;

const SERVICE_INBOX: u64 = 0;

pub fn loop_once(driver: Driver, rx: &mut [u8], tx: &mut [u8]) -> ! {
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        if sender_pid == 0x17 {
            debug::marker(b"recv compositor");
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else { continue };
        dispatch(&driver, sender_pid, req, body, tx);
    }
}
