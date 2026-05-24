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

use crate::protocol::{Request, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::server::respond;
use crate::state::INVARIANTS;

pub fn run(out: &mut [u8], req: &Request) -> usize {
    let count = INVARIANTS.len() as u32;
    let dst = HDR_LEN + STATUS_LEN;
    if dst + 4 > out.len() {
        return respond::status(out, req, E_INVAL);
    }
    out[dst..dst + 4].copy_from_slice(&count.to_le_bytes());
    let mut written = 4usize;
    for inv in INVARIANTS {
        let needed = 12 + inv.name.len() + inv.claim.len() + inv.mechanism.len();
        if dst + written + needed > out.len() {
            return respond::status(out, req, E_INVAL);
        }
        written += write_field(&mut out[dst + written..], inv.name);
        written += write_field(&mut out[dst + written..], inv.claim);
        written += write_field(&mut out[dst + written..], inv.mechanism);
    }
    respond::with_payload(out, req, 0, written)
}

fn write_field(out: &mut [u8], bytes: &[u8]) -> usize {
    out[0..4].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
    out[4..4 + bytes.len()].copy_from_slice(bytes);
    4 + bytes.len()
}
