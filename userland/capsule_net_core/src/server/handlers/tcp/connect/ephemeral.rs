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

use core::sync::atomic::{AtomicU16, Ordering};

const EPHEMERAL_BASE: u16 = 49152;
const EPHEMERAL_TOP: u16 = u16::MAX;
static EPHEMERAL: AtomicU16 = AtomicU16::new(EPHEMERAL_BASE);

pub fn next_ephemeral() -> u16 {
    let p = EPHEMERAL.fetch_add(1, Ordering::Relaxed);
    let range = EPHEMERAL_TOP - EPHEMERAL_BASE + 1;
    EPHEMERAL_BASE + p.wrapping_sub(EPHEMERAL_BASE) % range
}
