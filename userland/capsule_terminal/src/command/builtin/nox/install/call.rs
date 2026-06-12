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
use nonos_libc::mk_ipc_call;

use super::artifacts::Artifacts;

// Grant the loaded capsule its full declared optional caps. The verified
// manifest and the identity certificate ceiling are the real bounds; this only
// avoids restricting a capsule below what it legitimately declares.
const REQUESTED_CAPS: u64 = u64::MAX;
const EAGAIN: i32 = -11;

// Ask the installer to load a capsule from the store. The four artifacts are
// inlined behind a fixed header the installer forwards to the load syscall.
// Returns the new pid, or a negative status.
pub(super) fn call_installer(a: &Artifacts) -> Result<u32, i32> {
    let port = lookup_service(b"installer").map(|p| p.port).ok_or(EAGAIN)?;
    let payload = pack(a);
    let mut rx = [0u8; 32];
    let rc =
        mk_ipc_call(port as u64, payload.as_ptr(), payload.len(), rx.as_mut_ptr(), rx.len());
    if rc < 12 {
        return Err(EAGAIN);
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return Err(status);
    }
    Ok(u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]))
}

fn pack(a: &Artifacts) -> Vec<u8> {
    let mut p = Vec::with_capacity(24 + a.elf.len() + a.cert.len() + a.manifest.len() + a.trailer.len());
    p.extend_from_slice(&REQUESTED_CAPS.to_le_bytes());
    p.extend_from_slice(&(a.elf.len() as u32).to_le_bytes());
    p.extend_from_slice(&(a.cert.len() as u32).to_le_bytes());
    p.extend_from_slice(&(a.manifest.len() as u32).to_le_bytes());
    p.extend_from_slice(&(a.trailer.len() as u32).to_le_bytes());
    p.extend_from_slice(&a.elf);
    p.extend_from_slice(&a.cert);
    p.extend_from_slice(&a.manifest);
    p.extend_from_slice(&a.trailer);
    p
}
