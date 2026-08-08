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

//! Say it out loud, once, if the capsule store failed to decode at boot. The
//! probe retries every tick until vfs_pool answers, then latches: a corrupted
//! store used to present as a merely empty /capsules with nothing said.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

use nonos_libc::mk_time_millis;

use crate::render::sync_toast_layer;
use crate::state::{Context, NotifyLevel};

static ANSWERED: AtomicBool = AtomicBool::new(false);

pub fn check(ctx: &mut Context) {
    if ANSWERED.load(Ordering::Relaxed) {
        return;
    }
    let Some(code) = crate::vfs_client::store_status() else {
        return;
    };
    ANSWERED.store(true, Ordering::Relaxed);
    if code == 0 {
        return;
    }
    let mut text = Vec::with_capacity(32);
    text.extend_from_slice(b"capsule store corrupted (");
    crate::server::handlers::pkg_install::push_i32(&mut text, code as i32);
    text.push(b')');
    ctx.toasts.push(&text, NotifyLevel::Error, mk_time_millis());
    sync_toast_layer(ctx);
}
