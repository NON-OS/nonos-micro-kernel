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

#![no_std]
#![no_main]

extern crate alloc;

mod constants;
mod discover;
// The 802.11 frame layer is the first MLME brick above the alive firmware. It
// is not wired into the server path yet (the host-command queue and TX rings
// come next), so its builders are unused in the driver for now.
#[allow(dead_code)]
mod dot11;
mod driver;
mod firmware;
// The host-command queue above the alive firmware. The TFD-fill and DMA wiring
// that turn these into a real transfer come next, so the pieces are unused in
// the driver server path for now.
#[allow(dead_code)]
mod hcmd;
mod init;
mod protocol;
mod regs;
mod server;
mod setup;

use nonos_libc::{heap_init, mk_exit};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let Ok(driver) = setup::run() else {
        mk_exit(2);
    };
    server::run(driver);
}
