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

use nonos_libc::mk_time_millis;

use crate::protocol::{Request, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::server::respond;

pub fn run(out: &mut [u8], req: &Request) -> usize {
    let dst = HDR_LEN + STATUS_LEN;
    if dst + 24 > out.len() {
        return respond::status(out, req, E_INVAL);
    }
    let now_ms = read_time();
    let bootloader: &[u8] = b"NONOS bootloader (hybrid Ed25519 + ML-DSA-65)";
    out[dst..dst + 8].copy_from_slice(&now_ms.to_le_bytes());
    out[dst + 8..dst + 12].copy_from_slice(&(bootloader.len() as u32).to_le_bytes());
    let label_end = dst + 12 + bootloader.len();
    if label_end > out.len() {
        return respond::status(out, req, E_INVAL);
    }
    out[dst + 12..label_end].copy_from_slice(bootloader);
    respond::with_payload(out, req, 0, 12 + bootloader.len())
}

fn read_time() -> u64 {
    let raw = mk_time_millis();
    if raw < 0 {
        return 0;
    }
    raw as u64
}
