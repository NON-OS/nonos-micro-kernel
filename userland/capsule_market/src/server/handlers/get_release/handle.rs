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

use super::encode_release::encode_release;
use super::parse_pair::parse_pair;
use crate::protocol::{Request, E_INVAL, E_MSGSIZE, E_NODATA};
use crate::server::error::reply_status;
use crate::server::payload::{body_slot, reply_with_body};
use crate::store::Store;

pub(crate) fn handle(store: &Store, body: &[u8], req: &Request, tx: &mut [u8]) {
    let accepted = match store.current() {
        Some(a) => a,
        None => return reply_status(tx, req, E_NODATA),
    };
    let (listing_id, release_id) = match parse_pair(body) {
        Some(p) => p,
        None => return reply_status(tx, req, E_INVAL),
    };
    let release = accepted
        .index
        .entries
        .iter()
        .find(|e| e.listing_id == listing_id)
        .and_then(|e| e.releases.iter().find(|r| r.release_id == release_id));
    let release = match release {
        Some(r) => r,
        None => return reply_status(tx, req, E_NODATA),
    };
    let out = encode_release(release);
    let body_len = out.len();
    let slot = match body_slot(tx, body_len) {
        Some(s) => s,
        None => return reply_status(tx, req, E_MSGSIZE),
    };
    slot.copy_from_slice(&out);
    reply_with_body(tx, req, body_len);
}
