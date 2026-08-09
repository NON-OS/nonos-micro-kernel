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

//! OP_STORE_INSTALL, OP_STORE_UNINSTALL and OP_STORE_REMOVE are the ops that
//! create or destroy artifacts under the read-only /capsules tree, and holding
//! the vfs endpoint is not consent: any capsule that can talk to this server
//! could otherwise plant or downgrade an installed app behind the modal. The
//! service registry already binds the name "installer" to the pid the kernel
//! spawned it as, and `split_caller` hands us a kernel-attested sender pid, so
//! comparing the two is a real identity check and not a self-asserted one. The
//! lookup is per request because the installer registers after the vfs server
//! starts, and a respawn moves the pid.

use nonos_libc::mk_service_lookup;

use crate::protocol::EACCES;

const INSTALLER_SERVICE: &str = "installer";

// `sender_pid == 0` is the kernel-side mirror, the same TCB sentinel
// `split_caller` trusts to state its own caller pid.
pub(super) fn require_installer(sender_pid: u32) -> Result<(), i32> {
    if sender_pid == 0 {
        return Ok(());
    }
    match installer_pid() {
        Some(pid) if pid == sender_pid => Ok(()),
        _ => Err(EACCES),
    }
}

fn installer_pid() -> Option<u32> {
    let mut port: u32 = 0;
    let mut pid: u32 = 0;
    let rc = mk_service_lookup(
        INSTALLER_SERVICE.as_ptr(),
        INSTALLER_SERVICE.len(),
        &mut port,
        &mut pid,
    );
    if rc != 0 || pid == 0 {
        return None;
    }
    Some(pid)
}
