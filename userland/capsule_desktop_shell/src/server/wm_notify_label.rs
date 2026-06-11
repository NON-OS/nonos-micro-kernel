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

use nonos_libc::mk_service_lookup;

use crate::state::LAUNCHER_APPS;

pub(super) fn resolve_label(owner_pid: u32) -> Option<&'static [u8]> {
    for app in LAUNCHER_APPS.iter() {
        if service_pid(app.service) == Some(owner_pid) {
            return Some(app.label);
        }
    }
    None
}

fn service_pid(service: &[u8]) -> Option<u32> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(
        service.as_ptr(),
        service.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    if rc < 0 || pid == 0 {
        return None;
    }
    Some(pid)
}
