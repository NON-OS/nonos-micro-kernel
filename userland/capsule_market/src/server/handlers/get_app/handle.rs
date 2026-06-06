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

extern crate alloc;

use alloc::vec::Vec;

use super::read_lp_string::read_lp_string;
use super::write_lp_string::write_lp_string;
use crate::protocol::{Request, E_INVAL, E_MSGSIZE, E_NODATA};
use crate::server::error::reply_status;
use crate::server::payload::{body_slot, reply_with_body};
use crate::store::Store;

pub(crate) fn handle(store: &Store, body: &[u8], req: &Request, tx: &mut [u8]) {
    let accepted = match store.current() {
        Some(a) => a,
        None => return reply_status(tx, req, E_NODATA),
    };
    let listing_id = match read_lp_string(body) {
        Some(s) => s,
        None => return reply_status(tx, req, E_INVAL),
    };
    let entry = match accepted.index.entries.iter().find(|e| e.listing_id == listing_id) {
        Some(e) => e,
        None => return reply_status(tx, req, E_NODATA),
    };
    let mut out: Vec<u8> = Vec::new();
    write_lp_string(&mut out, &entry.listing_id);
    out.extend_from_slice(&entry.capsule_id);
    write_lp_string(&mut out, &entry.name);
    write_lp_string(&mut out, &entry.publisher_name);
    out.extend_from_slice(&entry.publisher_pubkey);
    write_lp_string(&mut out, &entry.description);
    out.extend_from_slice(&(entry.releases.len() as u32).to_le_bytes());
    let body_len = out.len();
    let slot = match body_slot(tx, body_len) {
        Some(s) => s,
        None => return reply_status(tx, req, E_MSGSIZE),
    };
    slot.copy_from_slice(&out);
    reply_with_body(tx, req, body_len);
}
