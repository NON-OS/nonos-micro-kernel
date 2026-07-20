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

mod device;
mod handles;
mod iface;
mod protocol;
mod register;
mod server;
mod setup;
mod state;
mod udp_ports;

use nonos_libc::{heap_init, mk_exit};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    // Register and serve before any interface is bound. The stack has no link at
    // boot (the WiFi link is down until the user associates and a laptop has no
    // cable), so binding is done opportunistically from the serve loop's periodic
    // re-evaluation. Registering first keeps net_core reachable while it waits, so
    // a client can see it is up but unbound instead of finding no service at all.
    register::all();
    server::run();
}
