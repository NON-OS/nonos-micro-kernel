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
// Last logged verdict per candidate: 0 unseen, 1 down, 2 no-answer. Indexed by
// position in the WiFi-then-wired order, so a change logs once, not per tick.
static PROBE_SEEN: [AtomicU32; 8] = [const { AtomicU32::new(0) }; 8];

fn discover_nic() -> Option<u32> {
    for (i, name) in WIFI_NICS.iter().chain(WIRED_NICS.iter()).enumerate() {
        let mut port: u32 = 0;
        let mut pid: u32 = 0;
        if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) != 0 {
            continue;
        }
        match device::link_up(port) {
            Some(true) => return Some(port),
            verdict => {
                let code = if verdict.is_none() { 2 } else { 1 };
                if i < PROBE_SEEN.len() && PROBE_SEEN[i].swap(code, Ordering::Relaxed) != code {
                    probe_log(name, verdict);
                }
            }
        }
    }
    None
}

// Which NIC was found and why it was not bound. A silent None here reads as a
// stack that never comes up, with every layer above reporting its own timeout.
fn probe_log(name: &str, verdict: Option<bool>) {
    let mut line = [0u8; 96];
    let tag: &[u8] = b"[NET-CORE] link probe ";
    let n = tag.len();
    line[..n].copy_from_slice(tag);
    let m = name.len().min(line.len() - n - 8);
    line[n..n + m].copy_from_slice(&name.as_bytes()[..m]);
    let tail: &[u8] = match verdict {
        Some(false) => b" down",
        None => b" no-answer",
        Some(true) => b" up",
    };
    line[n + m..n + m + tail.len()].copy_from_slice(tail);
    let _ = nonos_libc::mk_debug(line.as_ptr(), n + m + tail.len());
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
    // Both remaining exits used to be silent, and a stack that failed here
    // looked identical to one that found no link at all: no bind, no lease,
    // and every consumer reporting its own timeout.
    let Some(mac) = device::mac(best) else {
        bind_log(b"[NET-CORE] bind: mac query failed");
        return;
    };
    let Some(net_state) = build::build(mac, best) else {
        bind_log(b"[NET-CORE] bind: stack build failed");
        return;
    };
    state::store(net_state);
    BOUND_PORT.store(best, Ordering::Release);
    bind_log(b"[NET-CORE] bind: interface up");
}

fn bind_log(msg: &[u8]) {
    let _ = nonos_libc::mk_debug(msg.as_ptr(), msg.len());
}
