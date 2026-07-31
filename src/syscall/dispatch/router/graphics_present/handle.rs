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

/// Present a surface, or one damage rectangle of it. Zero width or height
/// means the whole surface, which is what a caller passing no rectangle sends.
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
