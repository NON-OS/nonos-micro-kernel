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
use super::rail_left_geom::head_h;
use super::rail_text::{lh, BAR_H, RAIL_GAP, RAIL_PAD};

pub const SPARK_H: u32 = 30;

/// The heights the telemetry painters advance by, kept apart from the painters
/// themselves so the scroll offset and the composer read one set of numbers and
/// can never disagree about where a section ends.
pub fn sys_h() -> u32 {
    head_h() + lh() * 5 + BAR_H + RAIL_GAP * 2 + SPARK_H
}

pub fn net_h() -> u32 {
    head_h() + lh() * 4
}

pub fn disk_h() -> u32 {
    head_h() + lh() * 2
}

/// The three sections stacked with one gap between each, plus the pad that
/// keeps the last row off the bottom edge of the rail.
pub fn telemetry_h() -> u32 {
    sys_h() + net_h() + disk_h() + RAIL_GAP * 2 + RAIL_PAD
}
