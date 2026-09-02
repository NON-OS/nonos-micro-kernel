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
use nonos_app_skeleton::PaintBuffer;

use super::rail_fmt::mib_into;
use super::rail_metric::one;
use super::rail_stat::stat;
use super::rail_text::head;
use crate::rail::disk::Disk;
use crate::term::theme::types::Theme;

pub use super::rail_geom::disk_h as height;

/// The mount rows. NONOS has no statfs and the capsule store is a raw signed
/// region rather than a volume, so both figures are standing gaps: the section
/// says so instead of the rail quietly omitting it.
pub fn draw(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, d: &Disk, t: &Theme) {
    let mut b = [0u8; 32];
    let mut y = head(fb, x, y, w, "DISK USAGE", t);
    y = stat(fb, x, y, w, "TOTAL", one(&mut b, d.total_kb, mib_into), t);
    stat(fb, x, y, w, "USED", one(&mut b, d.used_kb, mib_into), t);
}
