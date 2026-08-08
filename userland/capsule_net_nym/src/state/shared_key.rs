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

/// The key the current gateway registration produced. Held apart from the
/// Gateway record because every frame needs it while only connect sets it, and
/// it must clear on disconnect rather than linger for the next gateway.
static SHARED_KEY: Mutex<Option<[u8; 32]>> = Mutex::new(None);

pub fn set_gateway_shared_key(key: &[u8; 32]) {
    *SHARED_KEY.lock() = Some(*key);
}

pub fn gateway_shared_key() -> Option<[u8; 32]> {
    *SHARED_KEY.lock()
}

pub fn clear_gateway_shared_key() {
    *SHARED_KEY.lock() = None;
}
