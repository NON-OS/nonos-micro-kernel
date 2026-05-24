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

pub mod caps;
pub mod csr;
pub mod extensions;
pub mod fence;
pub mod halt;
pub mod id;
pub mod irq;
pub mod setup;

pub use csr::{clear_csr, read_csr, set_csr, write_csr};
pub use fence::{fence, fence_i, sfence_vma, sfence_vma_addr, sfence_vma_addr_asid, sfence_vma_asid};
pub use halt::{halt, wait_for_interrupt};
pub use id::{cpu_id, hart_id, marchid, mimpid, mvendorid};
pub use irq::{disable_interrupts, enable_interrupts, interrupts_enabled};
pub use setup::init_cpu;
