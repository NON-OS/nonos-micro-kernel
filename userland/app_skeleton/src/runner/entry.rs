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

use nonos_libc::{heap_init, mk_debug, mk_display_vsync_wait, mk_exit, HeapError};

use crate::app::App;
use crate::clients::compositor;
use crate::discover::require_peers;

use super::boot::{boot, paint_once};
use super::dispatch::DELIVERY_LEN;
use super::drain_ipc::drain;
use super::paint_frame::paint;
use super::request_id::next;
use super::teardown::close;

pub fn run<A: App, F: FnOnce() -> A>(build: F) -> ! {
    match heap_init() {
        Ok(()) | Err(HeapError::AlreadyInitialized) => {}
        Err(_) => fail(1, b"[app] heap fail\n"),
    }
    let _ = mk_debug(b"[app] start\n".as_ptr(), 12);
    let peers = match require_peers() {
        Ok(p) => p,
        Err(_) => fail(2, b"[app] peers fail\n"),
    };
    let app = build();
    let manifest = app.manifest();
    let mut request_id: u32 = 1;
    let mut booted = match boot(app, &peers, &mut request_id) {
        Ok(b) => b,
        Err(_) => fail_boot(3, manifest.title),
    };
    let mut rx = vec![0u8; DELIVERY_LEN.max(256)];
    loop {
        if !booted.primed {
            booted.primed = paint_once(
                &mut booted.app,
                &booted.manifest,
                &booted.binding,
                peers.toolkit,
                peers.compositor,
                &mut request_id,
            );
            if !booted.primed {
                let _ = mk_display_vsync_wait(0);
                continue;
            }
        }
        let result = drain(
            &mut booted.app,
            &mut rx,
            peers.wm,
            booted.manifest.window_id,
            &mut request_id,
        );
        if result.close {
            close(&peers, booted.manifest.window_id, &booted.binding, &mut request_id);
        }
        if result.repaint {
            let _ = paint(
                &mut booted.app,
                &booted.manifest,
                &booted.binding,
                peers.toolkit,
                &mut request_id,
            );
            let rid = next(&mut request_id);
            let _ = compositor::damage_commit(
                peers.compositor,
                rid,
                0,
                0,
                booted.manifest.width,
                booted.manifest.height,
            );
        }
        let _ = mk_display_vsync_wait(0);
    }
}

fn fail(code: i32, label: &[u8]) -> ! {
    let _ = mk_debug(label.as_ptr(), label.len());
    mk_exit(code)
}

fn fail_boot(code: i32, title: &[u8]) -> ! {
    let prefix = b"[app] boot fail: ";
    let mut buf = [0u8; 144];
    buf[..prefix.len()].copy_from_slice(prefix);
    let n = core::cmp::min(title.len(), buf.len() - prefix.len() - 1);
    buf[prefix.len()..prefix.len() + n].copy_from_slice(&title[..n]);
    buf[prefix.len() + n] = b'\n';
    fail(code, &buf[..prefix.len() + n + 1])
}
