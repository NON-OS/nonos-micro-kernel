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

use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use nonos_libc::mk_service_lookup;

use crate::device;
use crate::iface::build;
use crate::state;

// WiFi links are checked first. On a laptop the user explicitly associates, so an
// up WiFi link is the one they want; and a cable-less wired NIC can still claim
// carrier, which would strand the stack on a dead port. Wired links (a QEMU
// virtio-net or a real cabled NIC) are the fallback and still win when no WiFi
// link is up, so the desktop/QEMU path is unchanged.
const WIFI_NICS: &[&str] = &["driver.iwlwifi0", "driver.rtl8821ce0"];
const WIRED_NICS: &[&str] =
    &["driver.virtio_net0", "driver.e1000_0", "driver.rtl8169_0", "driver.rtl8139_0"];

// The broker port of the NIC the stack is currently bound to. A periodic
// re-evaluation compares against this to notice when a better link (the WiFi link
// coming up after boot) should replace the one bound at startup.
static BOUND_PORT: AtomicU32 = AtomicU32::new(0);

/// How many NIC drivers one discovery pass may block on.
const MAX_PROBES_PER_PASS: usize = 2;

/// Where the next discovery pass resumes, so a long candidate list is swept
/// across passes instead of inside one.
static PROBE_CURSOR: AtomicUsize = AtomicUsize::new(0);

/// The broker port of the interface the stack is bound to, or zero before the
/// first bind. Read by the lease-status reply so a panel can see which NIC the
/// stack chose when no address ever binds.
pub fn bound_port() -> u32 {
    BOUND_PORT.load(Ordering::Acquire)
}

// Return the first candidate whose link is actually up. Driver capsules register
// their service name at spawn, before probing hardware, so a wired NIC with no
// chip present or no cable attached still resolves through the service lookup.
// Binding the first name that merely exists stranded net_core on that dead
// interface: it reported link-down forever and net_core never fell through to the
// WiFi link, which only comes up once the user associates. Requiring an up link
// means net_core waits during boot and binds the WiFi interface the moment it
// connects, while still preferring a wired NIC that genuinely has carrier. A
// racing, dying wired capsule answers link_up with `None`, which is treated as
// not-up and skipped rather than taking net_core down.
// Each probe is a driver round trip the serve loop is blocked on, so a pass
// spends at most MAX_PROBES_PER_PASS of them and resumes at the next candidate
// a second later. Every real configuration registers one or two NIC drivers and
// so still sweeps the whole list in a single pass.
fn discover_nic() -> Option<u32> {
    let count = WIFI_NICS.len() + WIRED_NICS.len();
    let start = PROBE_CURSOR.load(Ordering::Relaxed);
    let mut probes = 0usize;
    for step in 0..count {
        let idx = (start + step) % count;
        let name = candidate(idx);
        let mut port: u32 = 0;
        let mut pid: u32 = 0;
        if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) != 0 {
            continue;
        }
        probes += 1;
        if device::link_up(port) == Some(true) {
            PROBE_CURSOR.store(idx, Ordering::Relaxed);
            return Some(port);
        }
        if probes == MAX_PROBES_PER_PASS {
            PROBE_CURSOR.store((idx + 1) % count, Ordering::Relaxed);
            return None;
        }
    }
    PROBE_CURSOR.store(0, Ordering::Relaxed);
    None
}

/// The NIC name at `idx`, counting WiFi candidates before wired ones.
fn candidate(idx: usize) -> &'static str {
    if idx < WIFI_NICS.len() {
        WIFI_NICS[idx]
    } else {
        WIRED_NICS[idx - WIFI_NICS.len()]
    }
}

/// Bind, or rebind, the stack to the best up link. The first call after boot with
/// a link up does the initial bind (nothing is bound, so any up link differs from
/// the zero sentinel); later calls switch interfaces when the WiFi link associates
/// or the bound link drops. A no-op once bound to the best link, so it is cheap to
/// call on a timer from the server loop.
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
