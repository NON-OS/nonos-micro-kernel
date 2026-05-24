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

pub mod context;
pub mod irq_handlers;
pub mod registers;

pub use context::{init_plic_hart, PlicContext};
pub use irq_handlers::{
    dispatch as dispatch_irq, register as register_irq_handler,
    register_for_capsule as register_irq_handler_for_capsule,
    unregister_for_capsule as unregister_irq_handler_for_capsule,
};
pub use registers::{
    claim_interrupt, complete_interrupt, disable_irq, enable_irq, init_plic, plic_present,
    set_priority, set_threshold, Plic, PlicError, PlicResult,
};
