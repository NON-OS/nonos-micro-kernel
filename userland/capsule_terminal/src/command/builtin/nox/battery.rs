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

// `battery`: charge percentage from the platform battery status. Negative means
// no battery is reported (a desktop, or emulation).

use alloc::vec::Vec;
use nonos_libc::mk_battery_status;

use crate::term::state::State;
use crate::term::util::format_u64;

pub fn run(state: &mut State) -> bool {
    let b = mk_battery_status();
    if b < 0 {
        state.scrollback.push_error(b"battery: not reported");
        return false;
    }
    let pct = (b as u64).min(100);
    let mut line: Vec<u8> = Vec::new();
    line.extend_from_slice(b"battery ");
    let mut buf = [0u8; 24];
    let k = format_u64(pct, &mut buf);
    line.extend_from_slice(&buf[..k]);
    line.push(b'%');
    state.scrollback.push_line(&line);
    true
}
