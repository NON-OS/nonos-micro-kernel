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

use core::sync::atomic::{AtomicU32, Ordering};
use nonos_libc::mk_service_lookup;

static NYM_PORT: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetupError {
    NymMissing,
}

/// Resolve the mixnet transport. Deliberately not `net.tcp`: this capsule
/// exists so that a stream leaves through the mixnet, and giving it a direct
/// socket would make a silent clearnet fallback possible.
pub fn run() -> Result<(), SetupError> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let name = b"net.nym";
    if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) != 0 || port == 0 {
        return Err(SetupError::NymMissing);
    }
    NYM_PORT.store(port, Ordering::Release);
    Ok(())
}

pub fn nym_port() -> u32 {
    NYM_PORT.load(Ordering::Acquire)
}
