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

use alloc::vec::Vec;
use spin::Mutex;

use crate::reply::{collect, open_reply, reply_body, Assembly};
use crate::state::TABLE;
use crate::trace;

/// Fragments of the reply currently being rebuilt.
///
/// One at a time is enough because a reply is answered before the next
/// request goes out, and holding several would mean deciding which of them a
/// fragment belongs to on nothing but its set id, which anyone could forge.
static PENDING: Mutex<Option<Assembly>> = Mutex::new(None);

/// Take a message the gateway pushed and deliver what it turns out to be.
///
/// Everything here is a filter, and a message that fails any of them is
/// dropped rather than passed on. A gateway pushes whatever it is handed, so
/// arriving is not evidence of anything: only opening under one of the reply
/// block keys we handed out is.
pub fn route_reply(payload: &[u8]) {
    trace::say_num(b"pushed message bytes", payload.len() as u64);
    let Some(fragment) = open_reply(payload) else {
        // Either it was not sealed to one of our blocks, or it is shorter
        // than the parts a reply is read in. Both mean it was not for us.
        trace::say(b"push dropped: no reply block key matched");
        return;
    };
    let Some(message) = collect(&mut PENDING.lock(), &fragment) else {
        trace::say(b"push held: message still missing fragments");
        return;
    };
    let Some(body) = reply_body(&message) else {
        trace::say_num(b"push dropped: not a reply message, bytes", message.len() as u64);
        return;
    };
    trace::say_num(b"reply delivered bytes", body.len() as u64);
    deliver(body);
}

/// Hand a reply to the session waiting on one.
///
/// A reply names no session: it came back on a block we handed out, and the
/// session that handed it out is the one holding a destination.
fn deliver(body: &[u8]) {
    let mut owned = Vec::with_capacity(body.len());
    owned.extend_from_slice(body);
    TABLE.lock().with_sphinx_session(|session| session.push(owned));
}
