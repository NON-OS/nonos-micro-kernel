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

use spin::Mutex;

/// The Nym address of the network requester that opens TCP on our behalf.
///
/// Not compiled in. An exit sees the hosts a client asks for, so which one to
/// trust is a decision for whoever runs the machine, and baking a default in
/// would make that choice silently on their behalf. Until it is set, connect
/// requests are refused rather than routed somewhere unchosen.
static EXIT: Mutex<Option<Exit>> = Mutex::new(None);

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
    let found =
        super::discover::discover_exit(0).or_else(|| super::bootstrap::bootstrap_exit(0))?;
    *slot = Some(found);
    Some(found)
}
