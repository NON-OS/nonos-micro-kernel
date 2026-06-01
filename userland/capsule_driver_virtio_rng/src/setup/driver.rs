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

//! `Driver` is the live state the server loop holds. It owns one
//! grant per broker primitive and releases them in the reverse
//! order on shutdown so the kernel sees a clean teardown even
//! when the capsule exits voluntarily.

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_irq_unbind};

use super::registers::RegisterGrant;
use crate::queue::Queue;
use crate::regs::Regs;

pub struct Driver {
    pub device_id: u64,
    pub claim_epoch: u64,
    pub(super) register_grant: RegisterGrant,
    pub irq_grant: u64,
    pub queue_grant: u64,
    pub buf_grant: u64,
    pub queue: Queue,
    pub regs: Regs,
}

impl Driver {
    /// Drop every grant the driver holds, in reverse order. Each
    /// broker call is best-effort; an `EINVAL` from a doubly-
    /// dropped grant is harmless because the broker has already
    /// revoked it on its side.
    pub fn release(&self) {
        let _ = mk_dma_unmap(self.buf_grant);
        let _ = mk_dma_unmap(self.queue_grant);
        let _ = mk_irq_unbind(self.irq_grant);
        let _ = self.register_grant.release();
        let _ = mk_device_release(self.device_id);
    }
}
