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
use nonos_libc::{mk_capsule_load, mk_getpid, CapsuleLoadRequest};

use crate::protocol::{encode_response, Request, EINVAL};

// A capsule ELF can be several MB. The artifacts are read here, one file at a
// time, so a multi-MB ELF never has to cross the IPC boundary in a single
// message: the caller sends only the name, and the verified blobs are read
// straight from the store.
const MAX_ARTIFACT: u32 = 16 * 1024 * 1024;

// Request payload: requested_caps(8) | name_len(1) | name | args.
//
// The caller hands us a store name, not the bytes. We read
// /capsules/<name>.{elf,nonos_id_cert.bin,manifest.bin,zk_trailer.bin}
// ourselves and pass them to the load syscall, which runs the full trust
// chain before the capsule is allowed to spawn.
// `sender_pid` is the kernel-attested pid of whoever sent this request
// (from `mk_ipc_recv_from`), never a value the payload carries. It becomes
// the spawned capsule's parent instead of this installer, so the requester
// can later `mk_wait`/`mk_kill`/`mk_proc_input`/`mk_proc_output` the child it
// asked for; honoring it kernel-side still requires this installer's own
// spawn-broker capability, so an ordinary capsule forging the same field
// through a raw `mk_capsule_load` call gets no effect.
pub fn load_by_name(req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let p = req.payload;
    if p.len() < 9 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let caps = u64::from_le_bytes([p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]]);
    let name_len = p[8] as usize;
    let Some(name_end) = 9usize.checked_add(name_len) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    if p.len() < name_end {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let name = &p[9..name_end];
    let args = &p[name_end..];
    if !valid_name(name) {
        return encode_response(req.seq, EINVAL, &[]);
    }

    let pid = mk_getpid();
    let (Ok(elf), Ok(cert), Ok(manifest), Ok(trailer)) = (
        read_artifact(pid, name, b".elf"),
        read_artifact(pid, name, b".nonos_id_cert.bin"),
        read_artifact(pid, name, b".manifest.bin"),
        read_artifact(pid, name, b".zk_trailer.bin"),
    ) else {
        return encode_response(req.seq, EINVAL, &[]);
    };

    let load = CapsuleLoadRequest {
        elf_ptr: elf.as_ptr() as u64,
        cert_ptr: cert.as_ptr() as u64,
        manifest_ptr: manifest.as_ptr() as u64,
        trailer_ptr: trailer.as_ptr() as u64,
        requested_caps: caps,
        args_ptr: if args.is_empty() { 0 } else { args.as_ptr() as u64 },
        elf_len: elf.len() as u32,
        cert_len: cert.len() as u32,
        manifest_len: manifest.len() as u32,
        trailer_len: trailer.len() as u32,
        args_len: args.len() as u32,
        on_behalf_of: sender_pid,
    };
    // The blobs above stay owned by this stack frame until the syscall
    // returns, so the kernel's copy reads live memory.
    let rc = mk_capsule_load(&load as *const CapsuleLoadRequest);
    if rc < 0 {
        return encode_response(req.seq, rc as i32, &[]);
    }
    encode_response(req.seq, 0, &(rc as u32).to_le_bytes())
}

fn read_artifact(pid: u32, name: &[u8], ext: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut path = Vec::with_capacity(10 + name.len() + ext.len());
    path.extend_from_slice(b"/capsules/");
    path.extend_from_slice(name);
    path.extend_from_slice(ext);
    read_file(pid, &path, MAX_ARTIFACT)
}

pub(super) fn valid_name(name: &[u8]) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}
