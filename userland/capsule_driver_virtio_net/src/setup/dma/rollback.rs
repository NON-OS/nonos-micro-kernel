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

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_irq_unbind, IrqBindOut};

use super::super::registers::RegisterGrant;

pub fn after(device_id: u64, reg: &RegisterGrant, irq: &IrqBindOut, grants: &[u64]) -> bool {
    let mut ok = true;
    for &grant in grants.iter().rev() {
        ok = mk_dma_unmap(grant) >= 0 && ok;
    }
    ok = mk_irq_unbind(irq.grant_id) >= 0 && ok;
    ok = reg.release() && ok;
    mk_device_release(device_id) >= 0 && ok
}
