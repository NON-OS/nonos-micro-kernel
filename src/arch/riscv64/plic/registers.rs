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

mod address;
mod api;
mod constants;
mod context_ops;
mod enable;
mod error;
mod init;
mod pending;
mod plic;
mod priority;
mod state;

pub use api::{claim_interrupt, complete_interrupt, disable_irq, enable_irq, set_priority, set_threshold};
pub use error::{PlicError, PlicResult};
pub use init::init_plic;
pub use plic::Plic;
pub use state::{current_plic, plic_present};
