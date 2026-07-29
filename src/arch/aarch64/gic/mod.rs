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

mod api;
pub mod cpu;
mod device;
pub mod distributor;
pub mod icc;
pub mod irq_handlers;
pub mod redistributor;
mod state;

pub use api::{disable_irq, enable_irq, init_gic, send_sgi, send_sgi_all_others};
pub use cpu::init_gic_cpu;
pub use device::Gic;
pub use distributor::GicDistributor;
pub use icc::{acknowledge_interrupt, end_interrupt, set_priority_mask};
pub use irq_handlers::{
    dispatch as dispatch_irq, register as register_irq_handler,
    register_for_capsule as register_irq_handler_for_capsule,
    unregister_for_capsule as unregister_irq_handler_for_capsule,
};
pub use redistributor::GicRedistributor;
