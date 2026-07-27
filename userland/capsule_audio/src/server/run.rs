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

use super::handle;
use super::pump::PumpState;
use super::streams::StreamTable;
use crate::mark::mark;
use crate::mixer::Mixer;
use crate::selftest;
use crate::sink::Sink;

const RX_LEN: usize = 4124;
const TX_LEN: usize = 28;
const RECV_TIMEOUT_MS: u64 = 5;

pub fn run() -> ! {
    mark("[AUDIO] up\n");
    let mut mixer = Mixer::new();
    let sink = Sink::resolve();
    if let Some(ref s) = sink {
        selftest::run_mix(&mut mixer, s);
        s.stream_start(3);
        crate::selftest_stream::run_streams(s);
        selftest::run(s);
    }
    let mut table = StreamTable::new();
    let mut pump = PumpState::new();
    let mut rx = vec![0u8; RX_LEN];
    let mut tx = vec![0u8; TX_LEN];
    loop {
        let n = mk_ipc_recv(0, rx.as_mut_ptr(), RX_LEN, RECV_TIMEOUT_MS);
        if n <= 0 {
            if let Some(ref s) = sink {
                super::pump::step(&mut pump, &mut table, s);
            }
            continue;
        }
        if let Some(ref s) = sink {
            handle(&rx[..n as usize], &mut mixer, s, &mut table, &mut pump, &mut tx);
        }
    }
}
