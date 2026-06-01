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

mod adopt;
mod constants;
mod disable_irq;
mod enable_irq;
mod gsi;
mod init;
mod pic;
mod regs;
mod set_irq;
mod state;

pub use disable_irq::disable_irq;
pub use enable_irq::enable_irq;
pub use init::init_ioapic;
pub use set_irq::ioapic_set_irq;
pub use state::IOAPIC_INIT;
