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

use super::seq;

pub fn should_update(snd_wl1: u32, snd_wl2: u32, seg_seq: u32, _snd_una: u32, seg_ack: u32) -> bool {
    seq::lt(snd_wl1, seg_seq) || (snd_wl1 == seg_seq && seq::leq(snd_wl2, seg_ack))
}

pub fn usable(snd_una: u32, snd_nxt: u32, snd_wnd: u32, cwnd: u32) -> u32 {
    let allowed = snd_wnd.min(cwnd);
    let in_flight = snd_nxt.wrapping_sub(snd_una);
    allowed.saturating_sub(in_flight)
}
