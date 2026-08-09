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

use nonos_app_skeleton::clients::vfs::read_file;
use nonos_libc::mk_getpid;

use super::pkg_body::parse_path;
use super::pkg_verify::{verified_name, verified_ns, verify_package, Verified};
use crate::protocol::{encode_response, Request, EINVAL};

pub(super) const MAX_PACKAGE: u32 = 16 * 1024 * 1024;

// Request payload: path_len(2, LE) | path.
// Reply payload: digest[32] | caps u64 LE | tier u8 | name_len u8 | name |
// ns_len u8 | ns. Hand-synced with the terminal and desktop_shell decoders.
pub fn pkg_query(req: Request<'_>) -> Vec<u8> {
    let Some(path) = parse_path(req.payload) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let Ok(bytes) = read_file(mk_getpid(), path, MAX_PACKAGE) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    match verify_package(&bytes) {
        Ok(v) => encode_response(req.seq, 0, &summary_payload(&v)),
        Err(status) => encode_response(req.seq, status, &[]),
    }
}

fn summary_payload(v: &Verified<'_>) -> Vec<u8> {
    let name = verified_name(&v.summary);
    let ns = verified_ns(&v.summary);
    let mut p = Vec::with_capacity(43 + name.len() + ns.len());
    p.extend_from_slice(&v.digest);
    p.extend_from_slice(&v.summary.caps.to_le_bytes());
    p.push(v.summary.tier);
    p.push(name.len() as u8);
    p.extend_from_slice(name);
    p.push(ns.len() as u8);
    p.extend_from_slice(ns);
    p
}
