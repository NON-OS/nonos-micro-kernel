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

//! Prove the 3D path at boot. When VirGL was negotiated: read capset 0,
//! create rendering context 1, and push a NOP command stream through
//! SUBMIT_3D with a fence — the same wire path every real render will take.
//! The result lands in the boot log either way, so a host without a GL
//! backend is a visible fact, not a silent downgrade.

use crate::device::cmd::{ctx_create, get_capset_info, submit_3d};
use crate::device::ControlQueue;
use crate::state::FenceCounter;

use nonos_libc::mk_debug;

// The context every in-OS 3D client shares until per-capsule contexts exist.
pub const RENDER_CTX_ID: u32 = 1;

// A VirGL command dword is (length << 16) | (object << 8) | command;
// VIRGL_CCMD_NOP is command 0 with no payload, so the stream is one zero
// dword. Round-tripping it proves decode, execution, and fence signalling.
const NOP_STREAM: [u8; 4] = [0, 0, 0, 0];

pub fn probe(q: &ControlQueue, fences: &FenceCounter, virgl: bool) -> bool {
    if !virgl {
        log(b"[GPU-3D] virgl not offered (2d scanout only)");
        return false;
    }
    let caps = match get_capset_info(q, 0) {
        Ok(c) => c,
        Err(e) => {
            log_err(b"[GPU-3D] capset query failed: ", e);
            return false;
        }
    };
    if let Err(e) = ctx_create(q, RENDER_CTX_ID, b"nonos-render") {
        log_err(b"[GPU-3D] context create failed: ", e);
        return false;
    }
    if let Err(e) = submit_3d(q, RENDER_CTX_ID, fences.issue(), &NOP_STREAM) {
        log_err(b"[GPU-3D] nop submit failed: ", e);
        return false;
    }
    log_caps(caps.capset_id, caps.max_version, caps.max_size);
    true
}

fn log(msg: &[u8]) {
    mk_debug(msg.as_ptr(), msg.len());
}

fn log_err(prefix: &[u8], e: &str) {
    let mut buf = [0u8; 96];
    let mut n = 0;
    push(&mut buf, &mut n, prefix);
    push(&mut buf, &mut n, e.as_bytes());
    mk_debug(buf.as_ptr(), n);
}

fn log_caps(id: u32, ver: u32, size: u32) {
    let mut buf = [0u8; 96];
    let mut n = 0;
    push(&mut buf, &mut n, b"[GPU-3D] virgl ready ctx=1 capset id=");
    push_dec(&mut buf, &mut n, id);
    push(&mut buf, &mut n, b" ver=");
    push_dec(&mut buf, &mut n, ver);
    push(&mut buf, &mut n, b" max=");
    push_dec(&mut buf, &mut n, size);
    mk_debug(buf.as_ptr(), n);
}

pub(super) fn push(buf: &mut [u8], n: &mut usize, bytes: &[u8]) {
    let take = bytes.len().min(buf.len() - *n);
    buf[*n..*n + take].copy_from_slice(&bytes[..take]);
    *n += take;
}

pub(super) fn push_dec(buf: &mut [u8], n: &mut usize, mut v: u32) {
    let mut tmp = [0u8; 10];
    let mut k = 0;
    loop {
        tmp[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k {
        push(buf, n, &tmp[k - 1 - i..k - i]);
    }
}
