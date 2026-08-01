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

use core::sync::atomic::{AtomicBool, Ordering};

use super::boot_session_nonce::init_boot_session_nonce;
use super::entropy::init_entropy;
use super::hardware_broker::seed_hardware_broker;
use super::token_signing_key::init_token_signing_key;

static DONE: AtomicBool = AtomicBool::new(false);

/// One-time platform setup that process creation depends on: the device
/// broker, the entropy source, the boot session nonce, and the capability
/// token signing key.
///
/// None of it is architecture specific, but it used to sit at the tail of the
/// x86_64 boot path, so any other entry reached process creation without a
/// nonce or a signing key and could not build a capability token. The guard
/// keeps a second caller from re-latching: the nonce reports an error rather
/// than silently replacing itself, and that error is fatal.
pub fn init_platform_baseline() {
    if DONE.swap(true, Ordering::SeqCst) {
        return;
    }
    // The identity mappings the boot map made for config space are gone by
    // now: bringing the unified address space up clears the low half. Anything
    // below that touches PCI would fault on a bare physical address.
    crate::arch::remap_pci_windows();
    // Before the scan, so the broker records devices at the addresses they
    // will actually decode at. Firmware that already assigned every BAR leaves
    // nothing to do here.
    let assigned = crate::bus::pci::assign_unassigned();
    if assigned > 0 {
        crate::sys::serial::print(b"[NONOS] PCI BARs assigned for ");
        crate::sys::serial::print_dec(assigned as u64);
        crate::sys::serial::println(b" devices");
    }
    // After the BARs are assigned, so the device table records the addresses
    // each device actually decodes at rather than the zeros it booted with.
    // The x86_64 path enumerated here from its own boot code, which no other
    // entry runs, so this is where every architecture gets a device table.
    crate::bus::pci::init();
    seed_hardware_broker();
    init_entropy();
    init_boot_session_nonce();
    init_token_signing_key();
}
