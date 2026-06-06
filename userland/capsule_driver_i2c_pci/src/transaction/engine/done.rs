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
use crate::constants::{IC_STATUS, IC_STATUS_MST_ACTIVITY, IC_STATUS_TFE};
use crate::regs::Regs;

pub fn done(regs: Regs) -> bool {
    let status = regs.read32(IC_STATUS);
    status & IC_STATUS_TFE != 0 && status & IC_STATUS_MST_ACTIVITY == 0
}
