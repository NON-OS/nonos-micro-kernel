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
use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::mk_ipc_call_timeout;

// Install reads the capsule's artifacts out of the store in chunks and runs
// the full verify and attestation before it replies, so it is inherently
// slower than an ordinary service call. A multi-megabyte binary can take a
// good few seconds; give the reply a wide budget rather than the 5s default,
// which a large capsule blows through while the load is still legitimately
// running.
const INSTALL_TIMEOUT_MS: u64 = 30_000;

// Grant the loaded capsule its full declared optional caps. The verified
// manifest and the identity certificate ceiling are the real bounds; this only
// avoids restricting a capsule below what it legitimately declares.
const REQUESTED_CAPS: u64 = u64::MAX;
const EAGAIN: i32 = -11;

// Installer wire: seq(4) | op(2) | pad(2) | body. Mirrors the installer's
// decode_request, which reads the op at byte 4 and the body after an 8 byte
// header. The reply is seq(4) | status(4) | pid(4), and the installer omits
// the pid on failure, so only a success reply is 12 bytes long.
const OP_LOAD_BY_NAME: u16 = 4;
const SEQ: u32 = 1;

// Ask the installer to load a store capsule by name. Only the name, caps and
// argv travel here; the installer reads the (multi-MB) artifacts from the
// store itself, so the message stays small. Returns the new pid, or a
// negative status.
pub(crate) fn call_installer(name: &[u8], args: &[u8]) -> Result<u32, i32> {
    let port = lookup_service(b"installer").map(|p| p.port).ok_or(EAGAIN)?;
    let payload = pack(name, args);
    let mut rx = [0u8; 32];
    let rc = mk_ipc_call_timeout(
        port as u64,
        payload.as_ptr(),
        payload.len(),
        rx.as_mut_ptr(),
        rx.len(),
        INSTALL_TIMEOUT_MS,
    );
    if rc < 8 {
        return Err(EAGAIN);
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return Err(status);
    }
    if rc < 12 {
        return Err(EAGAIN);
    }
    Ok(u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]))
}

fn pack(name: &[u8], args: &[u8]) -> Vec<u8> {
    let mut p = Vec::with_capacity(8 + 9 + name.len() + args.len());
    p.extend_from_slice(&SEQ.to_le_bytes());
    p.extend_from_slice(&OP_LOAD_BY_NAME.to_le_bytes());
    p.extend_from_slice(&[0u8, 0u8]);
    p.extend_from_slice(&REQUESTED_CAPS.to_le_bytes());
    p.push(name.len() as u8);
    p.extend_from_slice(name);
    p.extend_from_slice(args);
    p
}
