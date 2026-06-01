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

use nonos_abi::InputEvent;
use nonos_ipc::recv_from;

use super::super::wire::NINP_MAGIC;
use super::parse::parse_event;

const INBOX: u64 = 0;
const RECV_NOWAIT: u64 = 1;
const HDR: usize = 8;
const FRAME: usize = HDR + 32;

pub fn drain_input(out: &mut [InputEvent]) -> usize {
    let mut count = 0;
    let mut rx = [0u8; FRAME];
    while count < out.len() {
        let mut sender = 0u32;
        let n = recv_from(INBOX, &mut rx, RECV_NOWAIT, &mut sender);
        if n <= 0 || (n as usize) < FRAME {
            break;
        }
        if u32::from_le_bytes([rx[0], rx[1], rx[2], rx[3]]) != NINP_MAGIC {
            continue;
        }
        if let Some(event) = parse_event(&rx[HDR..FRAME]) {
            out[count] = event;
            count += 1;
        }
    }
    count
}
