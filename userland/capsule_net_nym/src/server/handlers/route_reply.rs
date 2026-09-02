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

use crate::reply::{collect, open_reply, reply_message, Assembly, Reply};
use crate::sphinx::constants::ACK_PLAINTEXT_SIZE;
use crate::state::TABLE;
use crate::trace;

/// Fragments of the reply currently being rebuilt.
///
/// One at a time is enough because a reply is answered before the next
/// request goes out, and holding several would mean deciding which of them a
/// fragment belongs to on nothing but its set id, which anyone could forge.
static PENDING: Mutex<Vec<Assembly>> = Mutex::new(Vec::new());

/// Take a message the gateway pushed and deliver what it turns out to be.
///
/// Everything here is a filter, and a message that fails any of them is
/// dropped rather than passed on. A gateway pushes whatever it is handed, so
/// arriving is not evidence of anything: only opening under one of the reply
/// block keys we handed out is.
pub fn route_reply(tcp_port: u32, payload: &[u8]) {
    trace::say_num(b"pushed message bytes", payload.len() as u64);
    // An acknowledgement is a fragment id under its own iv and nothing else,
    // so its width names it. It is not a reply and will not open as one. What
    // it says is that the fragment it names arrived, which is the only word
    // the far end sends back unprompted.
    if payload.len() == ACK_PLAINTEXT_SIZE {
        trace::say(b"fragment acknowledged by the far end");
        return;
    }
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
    match reply_message(&message) {
        Some(Reply::Data(body)) => {
            trace::say_num(b"reply delivered bytes", body.len() as u64);
            deliver(body);
        }
        // The far end has spent down to the reserve it keeps and will say
        // nothing more until it has room to answer. Everything sent after
        // this point depends on the top up going out.
        Some(Reply::SurbRequest { recipient, amount }) => {
            trace::say_num(b"far end asked for reply blocks", amount as u64);
            super::top_up::top_up(tcp_port, &recipient, amount);
        }
        None => {
            trace::say_num(b"push dropped: not a reply message, bytes", message.len() as u64);
        }
    }
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
