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

mod compositor_client;
mod debug;
mod focus;
mod geometry;
mod protocol;
mod server;
mod setup;
mod state;
mod window;
mod z_order;

use nonos_libc::{heap_init, mk_exit, mk_yield};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    debug::marker(b"boot");
    let ctx = wait_for_setup();
    debug::marker(b"setup complete");
    server::run(ctx);
}

fn wait_for_setup() -> crate::state::Context {
    let mut last: &'static str = "";
    loop {
        match setup::run() {
            Ok(ctx) => return ctx,
            Err(e) => {
                if e != last {
                    debug::marker(e.as_bytes());
                    last = e;
                }
                for _ in 0..64 {
                    mk_yield();
                }
            }
        }
    }
}
