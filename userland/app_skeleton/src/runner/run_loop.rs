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

use alloc::vec;

use nonos_libc::mk_time_millis;

use crate::app::App;
use crate::discover::require_peers;

use super::boot::boot;
use super::dispatch::DELIVERY_LEN;
use super::fail::fail;
use super::idle;
use super::repaint::repaint;
use super::service_frame::service_frame;

pub fn run_loop<A: App, F: Fn() -> A>(build: F) -> ! {
    let peers = match require_peers() {
        Ok(p) => p,
        Err(_) => fail(2, b"[app] peers fail\n"),
    };
    let mut request_id: u32 = 1;
    let mut rx = vec![0u8; DELIVERY_LEN.max(256)];
    loop {
        idle::wait(&mut rx);
        let app = build();
        let mut booted = match boot(app, &peers, &mut request_id) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let mut last_tick_ms: i64 = 0;
        loop {
            if service_frame(&mut booted, &mut rx, &peers, &mut request_id) {
                break;
            }
            let now = mk_time_millis();
            if now.wrapping_sub(last_tick_ms) >= booted.app.tick_interval_ms() {
                last_tick_ms = now;
                if booted.app.on_tick() && !booted.minimized {
                    repaint(&mut booted, &peers, &mut request_id);
                }
            }
        }
    }
}
