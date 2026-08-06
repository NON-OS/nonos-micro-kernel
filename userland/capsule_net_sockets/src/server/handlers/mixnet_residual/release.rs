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

use super::types::RESIDUAL;
use crate::sockets::SocketKey;

/// Drop anything held for a socket that is going away, so a later socket
/// reusing the handle does not read bytes meant for the one before it.
pub fn release(key: SocketKey) {
    let mut slots = RESIDUAL.lock();
    for slot in slots.iter_mut() {
        if slot.pid == key.pid && slot.handle == key.handle {
            slot.len = 0;
            slot.off = 0;
        }
    }
}
