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

use core::sync::atomic::{AtomicU32, Ordering};

use spin::Mutex;

use super::watch::Watch;

/// The Nym address of the network requester that opens TCP on our behalf.
///
/// Not compiled in. An exit sees the hosts a client asks for, so which one to
/// trust is a decision for whoever runs the machine, and baking a default in
/// would make that choice silently on their behalf. Until it is set, connect
/// requests are refused rather than routed somewhere unchosen.
static EXIT: Mutex<Option<Exit>> = Mutex::new(None);

/// Position in the directory's exit list. `find_exit` wraps it, so it only
/// ever grows; each rotation moves one node further along.
static INDEX: AtomicU32 = AtomicU32::new(0);

/// Delivery record for the exit in `EXIT`. See the module for the rule it
/// enforces: lookups prove nothing, only delivery does.
static WATCH: Mutex<Watch> = Mutex::new(Watch::new());

/// A network requester: 32-byte identity, 32-byte encryption key, and the
/// identity of the gateway it sits behind.
#[derive(Clone, Copy)]
pub struct Exit {
    pub identity: [u8; 32],
    pub encryption: [u8; 32],
    pub gateway: [u8; 32],
}

pub fn set_exit(exit: Exit) {
    *EXIT.lock() = Some(exit);
    let mut watch = WATCH.lock();
    watch.on_rotate();
    watch.configured = true;
}

/// A send left for the current exit.
pub fn note_sent() {
    WATCH.lock().on_send(nonos_libc::mk_uptime_ms());
}

/// A message came back through the current exit, which proves it.
pub fn note_delivered() {
    WATCH.lock().on_delivered();
}

/// Walk to the next exit if the current one has used up its silence budget.
///
/// Returns whether a rotation happened. The caller owns the consequences:
/// the session bound to the old exit is dead weight and has to be reopened,
/// and connections opened through it will never be answered.
pub fn rotate_if_silent() -> bool {
    let mut watch = WATCH.lock();
    if !watch.should_rotate(nonos_libc::mk_uptime_ms()) {
        return false;
    }
    watch.on_rotate();
    drop(watch);
    INDEX.fetch_add(1, Ordering::AcqRel);
    *EXIT.lock() = None;
    true
}

/// The exit to route through.
///
/// A choice made here wins. Otherwise one is taken from the directory, and
/// only if the network cannot be asked at all does the compiled list stand
/// in: that list ages, and an operator who stops running a requester leaves
/// every client that shipped with it unable to reach anything.
pub fn exit() -> Option<Exit> {
    let mut slot = EXIT.lock();
    if let Some(configured) = *slot {
        return Some(configured);
    }
    let index = INDEX.load(Ordering::Acquire);
    let found = super::discover::discover_exit(index)
        .or_else(|| super::bootstrap::bootstrap_exit(index as usize))?;
    *slot = Some(found);
    Some(found)
}
