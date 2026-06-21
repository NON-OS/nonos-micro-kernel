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

use crate::constants::regs::{PORT_IS, PORT_SERR};
use crate::regs::Regs;

pub(super) fn recover(regs: Regs, base: u32) {
    unsafe {
        regs.w32(base + PORT_SERR, regs.r32(base + PORT_SERR));
        regs.w32(base + PORT_IS, u32::MAX);
    }
    super::stop::stop(regs, base);
    super::start::start(regs, base);
}
