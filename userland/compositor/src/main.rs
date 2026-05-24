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

mod frame_pacer;
mod gfx_client;
mod protocol;
mod server;
mod setup;
mod state;
mod sw_blitter;

use nonos_libc::{heap_init, mk_exit, mk_service_register, mk_yield};

const SERVICE_NAME: &[u8] = b"compositor";
const SERVICE_PORT: u32 = 4310;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let ctx = wait_for_setup();
    if mk_service_register(SERVICE_NAME.as_ptr(), SERVICE_NAME.len(), SERVICE_PORT) < 0 {
        mk_exit(1);
    }
    server::run(ctx);
}

fn wait_for_setup() -> crate::state::Context {
    loop {
        match setup::run() {
            Ok(ctx) => return ctx,
            Err(_) => {
                wait_after_error();
            }
        }
    }
}

fn wait_after_error() {
    for _ in 0..64 {
        mk_yield();
    }
}
