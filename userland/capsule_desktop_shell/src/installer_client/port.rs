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

//! Resolve the installer's endpoint by service name. None means the service is
//! not registered, which is the normal state on a system without a store.

use core::ptr;

use nonos_libc::mk_service_lookup;

use super::constants::SERVICE;

pub(super) fn port() -> Option<u32> {
    let mut port = 0u32;
    let rc =
        mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port as *mut u32, ptr::null_mut());
    if rc < 0 || port == 0 {
        return None;
    }
    Some(port)
}
