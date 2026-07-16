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

//! RTL8821CE wifi driver, cold-start stage. This first stage claims the chip,
//! maps its registers, runs the proven power-on sequence and reads the chip
//! back, reporting on the boot console how far it got. The firmware download,
//! MAC/PHY bring-up and RF calibration build on this once the chip is proven
//! to answer on real silicon.

#![no_std]
#![no_main]

extern crate alloc;

mod assoc;
mod bringup;
mod constants;
mod discover;

mod efuse;
mod fw;
mod fwload;
mod link;
mod mac;
mod phy;
mod pwr;
mod regs;
mod ring;
mod rx;
mod scan;
mod sec;
mod serve;
mod setup;
mod status;
mod tx;

use nonos_libc::{heap_init, mk_exit};

use bringup::{probe, BringUp};
use serve::Stage;
use setup::Mapped;

/// # Safety
/// The capsule entry point. The runtime jumps here once, on a fresh stack, with
/// no prior Rust state; it must not be called from Rust. It initialises the heap
/// before any allocation and never returns.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    // Bring the radio up as far as it will go, then serve regardless of how far
    // that was. Serving keeps the service registered so the panel can read the
    // stage; exiting or hanging here would drop the service and show as nothing.
    let (mapped, stage) = bring_up();
    serve::run(mapped, stage);
}

/// Claim the device, power the MAC on, load the firmware and initialise the MAC,
/// stopping at the first step that fails and reporting how far it got. On a
/// serial-less machine the returned stage is the only way to see this, surfaced
/// through the panel's status request.
fn bring_up() -> (Option<Mapped>, Stage) {
    let mapped = match setup::run() {
        Ok(m) => m,
        Err(e) => {
            status::line(b"[rtl8821ce] ");
            status::line(e.as_bytes());
            status::line(b"\n");
            return (None, Stage::NotClaimed);
        }
    };
    match probe(&mapped.regs) {
        Ok(_id) => status::line(b"[rtl8821ce] chip powered and answering\n"),
        Err(BringUp::PowerFailed) => {
            status::line(b"[rtl8821ce] power-on sequence failed\n");
            return (Some(mapped), Stage::PowerFailed);
        }
        Err(BringUp::DeadMmio) => {
            status::line(b"[rtl8821ce] no response after power-on\n");
            return (Some(mapped), Stage::DeadMmio);
        }
    }
    // With the MAC powered, load the 8051 firmware. On real silicon this runs the
    // reserved-page staging and DDMA; off-silicon the whole path bar the DMA
    // mapping and the card's own execution is proven in rtl8821ce_proofs.
    if fwload::load(&mapped.regs, mapped.device_id, mapped.claim_epoch).is_err() {
        status::line(b"[rtl8821ce] firmware download failed\n");
        return (Some(mapped), Stage::FirmwareFailed);
    }
    status::line(b"[rtl8821ce] firmware loaded and ready\n");
    // Switch the transmit and receive engines on: queue mapping, the reserved
    // page boundaries and link-list table, and the H2C queue. Without this the
    // register table below configures the receiver but it never runs, so the
    // radio comes up yet hears nothing.
    if !mac::init_trx_cfg(&mapped.regs) {
        status::line(b"[rtl8821ce] trx engine enable failed\n");
        return (Some(mapped), Stage::FirmwareFailed);
    }
    status::line(b"[rtl8821ce] trx engines enabled\n");
    // Register work only: protocol, EDCA, beacon and WMAC configuration.
    mac::run_mac_table(&mapped.regs, mac::MAC_INIT);
    status::line(b"[rtl8821ce] mac initialised\n");
    (Some(mapped), Stage::Ready)
}
