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
use super::reply::send_reply;
use super::transfer::do_transfer;
use crate::protocol::{Request, CONTROL_TRANSFER_REQUEST_LEN, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;

// Largest data stage this handler will accept. The reply is copied into the
// fixed-size `tx` buffer, so w_len must be clamped or a client-chosen w_len up
// to 65535 overflows tx (heap OOB write). 512 matches the config-descriptor cap
// and leaves ample headroom in tx (512 + header < 2068).
const CONTROL_TRANSFER_DATA_MAX: u16 = 512;

pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != CONTROL_TRANSFER_REQUEST_LEN {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let (slot, bm_rt, b_req) = (body[0], body[2], body[3]);
    let w_value = u16::from_le_bytes([body[4], body[5]]);
    let w_index = u16::from_le_bytes([body[6], body[7]]);
    let w_len = u16::from_le_bytes([body[8], body[9]]).min(CONTROL_TRANSFER_DATA_MAX);
    let region = if w_len > 0 {
        match ctx.driver.dma_pool.alloc(w_len as u64) {
            Ok(r) => {
                r.zero();
                Some(r)
            }
            Err(_) => {
                reply_with_status(tx, req, E_IO);
                return;
            }
        }
    } else {
        None
    };
    let (data_phys, data_len) = region.as_ref().map_or((0u64, 0u16), |r| (r.phys(), w_len));
    match do_transfer(ctx, slot, bm_rt, b_req, w_value, w_index, data_len, data_phys) {
        Ok(()) => send_reply(tx, req, data_len, region.as_ref()),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
