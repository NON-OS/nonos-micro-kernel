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

use crate::transport::State;

pub fn state_label(s: State) -> &'static [u8] {
    match s {
        State::Playing => b"Playing",
        State::Paused => b"Paused",
        State::Stopped => b"Stopped",
    }
}

fn mmss(buf: &mut [u8], off: usize, ms: u32) -> usize {
    let total = ms / 1000;
    let mm = total / 60;
    let ss = total % 60;
    buf[off] = b'0' + ((mm / 10) % 10) as u8;
    buf[off + 1] = b'0' + (mm % 10) as u8;
    buf[off + 2] = b':';
    buf[off + 3] = b'0' + (ss / 10) as u8;
    buf[off + 4] = b'0' + (ss % 10) as u8;
    off + 5
}

pub fn time_readout(buf: &mut [u8], pos_ms: u32, dur_ms: u32) -> usize {
    let mut n = mmss(buf, 0, pos_ms);
    buf[n] = b' ';
    buf[n + 1] = b'/';
    buf[n + 2] = b' ';
    n += 3;
    mmss(buf, n, dur_ms)
}
