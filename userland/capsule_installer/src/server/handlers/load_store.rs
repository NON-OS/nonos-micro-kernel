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

use crate::protocol::{encode_response, Request, EINVAL};
use nonos_libc::{mk_capsule_load, CapsuleLoadRequest};

// on_behalf_of below is intentionally left 0 (unattributed): this handler is
// the direct VFS-store load path, not the broker-attributed spawn path.
pub fn load_store(req: Request<'_>) -> Vec<u8> {
    const HEAD_LEN: usize = 28;
    if req.payload.len() < HEAD_LEN {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let elf_len = u32::from_le_bytes([p[8], p[9], p[10], p[11]]) as usize;
    let cert_len = u32::from_le_bytes([p[12], p[13], p[14], p[15]]) as usize;
    let manifest_len = u32::from_le_bytes([p[16], p[17], p[18], p[19]]) as usize;
    let trailer_len = u32::from_le_bytes([p[20], p[21], p[22], p[23]]) as usize;
    let args_len = u32::from_le_bytes([p[24], p[25], p[26], p[27]]) as usize;
    let Some(after_elf) = HEAD_LEN.checked_add(elf_len) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let Some(after_cert) = after_elf.checked_add(cert_len) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let Some(after_manifest) = after_cert.checked_add(manifest_len) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let Some(after_trailer) = after_manifest.checked_add(trailer_len) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let Some(total) = after_trailer.checked_add(args_len) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    if p.len() != total {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let load = CapsuleLoadRequest {
        elf_ptr: p[HEAD_LEN..after_elf].as_ptr() as u64,
        cert_ptr: p[after_elf..after_cert].as_ptr() as u64,
        manifest_ptr: p[after_cert..after_manifest].as_ptr() as u64,
        trailer_ptr: p[after_manifest..after_trailer].as_ptr() as u64,
        requested_caps: u64::from_le_bytes([p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]]),
        args_ptr: arg_ptr(p, after_trailer, total),
        elf_len: elf_len as u32,
        cert_len: cert_len as u32,
        manifest_len: manifest_len as u32,
        trailer_len: trailer_len as u32,
        args_len: args_len as u32,
        on_behalf_of: 0,
    };
    let rc = mk_capsule_load(&load as *const CapsuleLoadRequest);
    if rc < 0 {
        return encode_response(req.seq, rc as i32, &[]);
    }
    encode_response(req.seq, 0, &(rc as u32).to_le_bytes())
}

fn arg_ptr(p: &[u8], start: usize, total: usize) -> u64 {
    if start == total { 0 } else { p[start..total].as_ptr() as u64 }
}
