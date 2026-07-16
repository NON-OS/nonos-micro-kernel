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

use crate::device;
use crate::iface::build;
use crate::state;

const NIC_CANDIDATES: &[&str] =
    &["driver.virtio_net0", "driver.e1000_0", "driver.rtl8169_0", "driver.rtl8139_0"];

// The broker port of the NIC the stack is currently bound to. A periodic
// re-evaluation compares against this to notice when a better link (a NIC that
// gained carrier after boot) should replace the one bound at startup.
static BOUND_PORT: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetupError {
    NicNotFound,
    MacFailed,
    BuildFailed,
}

// Return the first candidate whose link is actually up. A driver registers its
// service name at spawn, before it probes, so a NIC with no chip present or no
// cable attached still resolves through the lookup and, worse, can report no
// carrier; binding the first name that merely exists stranded the stack on that
// dead interface. Requiring an up link, and re-checking on a timer, means the
// stack binds the interface that has carrier and follows it if that changes.
// A racing, dying capsule answers link_up with None, treated as not-up.
fn discover_nic() -> Option<u32> {
    for name in NIC_CANDIDATES {
        let mut port: u32 = 0;
        let mut pid: u32 = 0;
        if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) == 0
            && device::link_up(port) == Some(true)
        {
            return Some(port);
        }
    }
    None
}

pub fn run() -> Result<(), SetupError> {
    let port = discover_nic().ok_or(SetupError::NicNotFound)?;
    let mac = device::mac(port).ok_or(SetupError::MacFailed)?;
    let net_state = build::build(mac, port).ok_or(SetupError::BuildFailed)?;
    state::store(net_state);
    BOUND_PORT.store(port, Ordering::Release);
    Ok(())
}

/// Re-check the interface picture after the initial bind. When the best up link
/// differs from the one currently bound - a NIC gaining carrier after boot, or
/// the bound link going down - rebuild the stack on the new interface so DHCP
/// runs where traffic can actually flow. A no-op once bound to the best link, so
/// it is cheap to call on a timer from the server loop.
pub fn reevaluate() {
    let Some(best) = discover_nic() else {
        return;
    };
    if best == BOUND_PORT.load(Ordering::Acquire) {
        return;
    }
    let Some(mac) = device::mac(best) else {
        return;
    };
    let Some(net_state) = build::build(mac, best) else {
        return;
    };
    state::store(net_state);
    BOUND_PORT.store(best, Ordering::Release);
}
