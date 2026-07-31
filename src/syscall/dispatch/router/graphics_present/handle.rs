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

use super::blit::blit;
use crate::syscall::SyscallResult;

/// Present a surface, either whole or one damage rectangle of it.
///
/// A zero width or height means "the whole thing", which is what a caller that
/// passes no rectangle at all sends. That keeps the meaning of the old
/// four-argument call intact while letting a compositor that knows what it
/// changed say so: on the GOP path the blit is a CPU copy of every byte it is
/// given, so a full screen is 8.3 MB at 1920x1080 whether one pixel moved or
/// all of them did.
pub(in crate::syscall::dispatch::router) fn handle(
    display: u64,
    surface: u64,
    span: usize,
    x: u64,
    y: u64,
    w: u64,
    h: u64,
) -> SyscallResult {
    let full = w == 0 || h == 0;
    blit(display, surface, span, x, y, w, h, full)
}
