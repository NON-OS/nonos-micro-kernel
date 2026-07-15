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

use nonos_libc::{mk_irq_bind, IrqBindOut};

use crate::discover::Found;

/// Bind the controller interrupt when the platform offers a usable one. The
/// transfer engine polls, so this is best-effort: a controller that declares no
/// interrupt (irq_line 0xFF) or whose GSI does not translate on this platform
/// still drives the bus. A zero grant means no interrupt was bound.
pub fn bind(dev: Found, claim_epoch: u64) -> IrqBindOut {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    if mk_irq_bind(dev.device_id, claim_epoch, dev.irq_line as u32, 0, 0, &mut out) < 0 {
        return IrqBindOut { grant_id: 0, vector: 0 };
    }
    out
}
