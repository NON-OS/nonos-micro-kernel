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

use super::stream::restart;
use crate::protocol::{Request, E_INVAL, E_OK, HDR_LEN, MAX_PCM_CHUNK};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle(driver: &Driver, req: &Request, msg: &[u8], tx: &mut [u8], played: &mut bool) {
    let want = req.payload_len as usize;
    let avail = msg.len().saturating_sub(HDR_LEN);
    if want == 0 || want > MAX_PCM_CHUNK || want > avail {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    copy_pcm(driver, &msg[HDR_LEN..HDR_LEN + want]);
    restart(driver);
    *played = false;
    reply_with_status(tx, req, E_OK);
}

fn copy_pcm(driver: &Driver, pcm: &[u8]) {
    let cap = driver.sample.length as usize;
    let n = pcm.len().min(cap);
    let dst = driver.sample.user_va as *mut u8;
    let mut i = 0usize;
    while i < n {
        unsafe { write_volatile(dst.add(i), pcm[i]) };
        i += 1;
    }
    while i < cap {
        unsafe { write_volatile(dst.add(i), 0u8) };
        i += 1;
    }
}
