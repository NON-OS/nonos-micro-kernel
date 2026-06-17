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

//! Control-op authorization. A sensitive op is allowed only when the
//! kernel-attested `sender_pid` matches the owner pid the registry
//! bound to an authorized caller service. Lookup miss or pid 0 rejects.

use nonos_libc::mk_service_lookup;

const ADMIN_SERVICE: &[u8] = b"net.admin";

fn owner_pid(service: &[u8]) -> Option<u32> {
    let mut pid: u32 = 0;
    let rc = mk_service_lookup(service.as_ptr(), service.len(), core::ptr::null_mut(), &mut pid);
    if rc != 0 || pid == 0 {
        return None;
    }
    Some(pid)
}

pub fn authorized(sender_pid: u32, service: &[u8]) -> bool {
    owner_pid(service) == Some(sender_pid)
}

/// Tier-2 administrative ops. No capsule is spawned as `net.admin`
/// today, so the lookup misses and every caller is denied
/// (deny-by-default). A future admin/settings principal registered
/// under that name enables these ops with no handler change.
pub fn admin(sender_pid: u32) -> bool {
    authorized(sender_pid, ADMIN_SERVICE)
}
