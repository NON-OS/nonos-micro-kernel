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

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use super::super::addr20::addr20;
use super::super::discover::keyring_port;
use super::super::epoch::current_epoch;
use super::super::expiry::expiry_at;
use super::super::fields::ReceiptInput;
use super::super::record::build_record;
use super::super::sign_call::sign_receipt;
use super::super::u64_word::u64_word;
use super::super::word32::word32;
use crate::protocol::{encode_response, Request, EAGAIN, EINVAL};
use crate::store::State;

pub fn pay(state: &mut State, req: Request<'_>) -> Vec<u8> {
    const HDR: usize = 4 + 4 + 32 + 20 + 32 + 32;
    if req.payload.len() != HDR {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let owner_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let wallet_id = u32::from_le_bytes([p[4], p[5], p[6], p[7]]);
    let publisher = addr20(p, 40);
    let now_ms = mk_time_millis();
    let port = match keyring_port() {
        Some(x) => x,
        None => return encode_response(req.seq, EAGAIN, &[]),
    };
    if now_ms <= 0 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let now_secs = (now_ms as u64) / 1000;
    let nonce = state.next_nonce(owner_pid, wallet_id, &publisher);
    let f = ReceiptInput {
        capsule_id: word32(p, 8),
        publisher,
        amount: word32(p, 60),
        nonce: u64_word(nonce),
        epoch: u64_word(current_epoch(now_secs)),
        expiry: u64_word(expiry_at(now_secs)),
        receipt_type: word32(p, 92),
    };
    let signed = match sign_receipt(port, owner_pid, wallet_id, &f) {
        Ok(s) => s,
        Err(e) => return encode_response(req.seq, e, &[]),
    };
    if !state.push_receipt(build_record(&f, &signed)) {
        return encode_response(req.seq, EINVAL, &[]);
    }
    encode_response(req.seq, 0, &signed.struct_hash)
}
