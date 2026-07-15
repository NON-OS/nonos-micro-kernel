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

use nonos_libc::{mk_irq_bind, IrqBindOut, MK_IRQ_BIND_MSIX};

use crate::discover::Found;

pub fn bind(dev: Found, claim_epoch: u64) -> IrqBindOut {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    let r = mk_irq_bind(dev.device_id, claim_epoch, 0, MK_IRQ_BIND_MSIX, 1, &mut out);
    if r < 0 {
        // MSI-X binding is best effort. The driver polls every admin and I/O
        // completion (admin/queue/wait.rs, nvm/wait.rs) and never waits on the
        // interrupt, so a failed bind is not fatal. Continue in polling mode
        // with a zero grant, which BrokerHandles::drop unbinds harmlessly.
        return IrqBindOut { grant_id: 0, vector: 0 };
    }
    out
}
