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

use nonos_app_skeleton::clients::vfs::{read_file, store_install};
use nonos_libc::mk_getpid;

use super::super::discover::service_taken;
use super::load_by_name::valid_name;
use super::pkg_body::parse_commit;
use super::pkg_paths::{artifact_path, installed, EXTS};
use super::pkg_query::MAX_PACKAGE;
use super::pkg_verify::{install_name, verify_package, EACCES};
use crate::protocol::{encode_response, Request, EINVAL};

const EEXIST: i32 = -17;
const EIO: i32 = -5;

// Request payload: digest[32] | path_len(2, LE) | path; empty success payload.
// The package is re-read, re-parsed and re-verified here, and the recomputed
// digest must match the one the caller consented to, so a swap between the
// query that produced the consent prompt and this commit is rejected. The
// install name is derived from the verified namespace, never from the file
// name the caller supplied.
pub fn pkg_commit(req: Request<'_>) -> Vec<u8> {
    let Some((digest, path)) = parse_commit(req.payload) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let pid = mk_getpid();
    let Ok(bytes) = read_file(pid, path, MAX_PACKAGE) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let v = match verify_package(&bytes) {
        Ok(v) => v,
        Err(status) => return encode_response(req.seq, status, &[]),
    };
    if v.digest != *digest {
        return encode_response(req.seq, EACCES, &[]);
    }
    let name = install_name(&v.summary);
    if !valid_name(name) {
        return encode_response(req.seq, EINVAL, &[]);
    }
    if installed(pid, name) || service_taken(name) {
        return encode_response(req.seq, EEXIST, &[]);
    }
    let blobs = [v.sections.elf, v.sections.manifest, v.sections.id_cert, v.sections.zk_trailer];
    for (ext, blob) in EXTS.iter().zip(blobs) {
        let path = artifact_path(name, ext);
        if store_install(&path, blob).is_err() {
            return encode_response(req.seq, EIO, &[]);
        }
    }
    encode_response(req.seq, 0, &[])
}
