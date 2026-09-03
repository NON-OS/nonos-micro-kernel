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
use crate::protocol::{OP_FLUSH, OP_WRITE_BLOCKS};

// The one service name the write path admits besides the kernel client. The
// kernel records it at spawn, after the spawn gate verified the capsule's
// certificate, manifest and membership trailer against the baked policy
// root, so the name is exactly as strong as the attestation chain.
pub const INSTALLER_NAME: &[u8] = b"app.nonos_install";

// The whole decision, pure so the host proofs can walk every case. The
// sender pid is stamped by the kernel and pid 0 is never handed to a
// process, so it marks the kernel-internal client and nothing else.
pub fn allows(op: u16, sender_pid: u32, sender_is_installer: bool) -> bool {
    match op {
        OP_WRITE_BLOCKS | OP_FLUSH => sender_pid == 0 || sender_is_installer,
        _ => true,
    }
}

// An entry names the installer only on an exact match: right length, right
// bytes. A longer name that merely starts with the installer's must fail.
pub fn entry_names_installer(name: &[u8], name_len: u8) -> bool {
    let n = name_len as usize;
    n == INSTALLER_NAME.len() && n <= name.len() && &name[..n] == INSTALLER_NAME
}
