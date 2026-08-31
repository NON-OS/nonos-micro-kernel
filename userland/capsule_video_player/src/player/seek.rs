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

pub fn frame_at_permille(total: u32, permille: u32) -> u32 {
    if total == 0 {
        return 0;
    }
    let f = permille.min(1000) as u64 * total as u64 / 1000;
    (f as u32).min(total - 1)
}

pub fn permille_of(current: u32, total: u32) -> u32 {
    if total == 0 {
        return 0;
    }
    ((current as u64 * 1000) / total as u64).min(1000) as u32
}

pub fn frame_after_delta(current: u32, total: u32, usec_per_frame: u32, delta_secs: i32) -> u32 {
    if total == 0 || usec_per_frame == 0 {
        return current;
    }
    let per_sec = (1_000_000i64 / usec_per_frame as i64).max(1);
    let delta = (delta_secs as i64).saturating_mul(per_sec);
    let next = (current as i64).saturating_add(delta);
    next.clamp(0, total as i64 - 1) as u32
}

pub fn duration_ms(total: u32, usec_per_frame: u32) -> i64 {
    (total as i64).saturating_mul(usec_per_frame as i64) / 1000
}
