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

use nonos_app_skeleton::clients::vfs::{stat, store_uninstall};
use nonos_libc::mk_getpid;

use super::load_by_name::valid_name;
use super::pkg_body::parse_name;
use super::pkg_paths::{artifact_path, EXTS};
use crate::protocol::{encode_response, Request, EINVAL};

const ENOENT: i32 = -2;
const EIO: i32 = -5;

// Request payload: name_len(2, LE) | name; empty success payload. A partial
// install is removed too: any present subset is deleted, and only a fully
// absent name is ENOENT. Each artifact is dropped from the RAM tree and from
// the packaged on-device store, so a reboot does not resurrect it.
pub fn pkg_remove(req: Request<'_>) -> Vec<u8> {
    let Some(name) = parse_name(req.payload) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    if !valid_name(name) {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let pid = mk_getpid();
    let present: Vec<Vec<u8>> = EXTS
        .iter()
        .map(|ext| artifact_path(name, ext))
        .filter(|path| stat(pid, path).is_ok())
        .collect();
    if present.is_empty() {
        return encode_response(req.seq, ENOENT, &[]);
    }
    for path in &present {
        if store_uninstall(path).is_err() {
            return encode_response(req.seq, EIO, &[]);
        }
    }
    encode_response(req.seq, 0, &[])
}
