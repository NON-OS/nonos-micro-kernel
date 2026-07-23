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

use core::ptr::write_volatile;

use crate::controller::{stream_run, StreamDescriptor, StreamRun};
use crate::protocol::{Request, E_OK};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle_stream_start(driver: &Driver, req: &Request, tx: &mut [u8], running: &mut bool) {
    if !*running {
        silence(driver);
        restart(driver);
        *running = true;
    }
    reply_with_status(tx, req, E_OK);
}

fn silence(driver: &Driver) {
    let dst = driver.sample.user_va as *mut u8;
    let cap = driver.sample.length as usize;
    let mut i = 0usize;
    while i < cap {
        unsafe { write_volatile(dst.add(i), 0u8) };
        i += 1;
    }
}

pub(super) fn restart(driver: &Driver) {
    let desc = StreamDescriptor {
        kind: 0,
        local_index: 0,
        global_index: driver.stream_gi,
        mmio_offset: driver.stream_off,
    };
    stream_run::run(
        driver.regs,
        StreamRun {
            desc,
            tag: driver.stream_tag,
            bdl_va: driver.bdl.user_va,
            bdl_dev: driver.bdl.device_addr,
            sample_dev: driver.sample.device_addr,
            bytes: driver.sample.length as u32,
        },
    );
}
