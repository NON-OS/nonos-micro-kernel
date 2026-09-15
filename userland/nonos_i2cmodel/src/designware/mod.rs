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

//! The DesignWare I2C master inside an Intel LPSS wrapper, register by
//! register, from the databook.

mod config;
mod exec;
mod fifo;
mod inspect;
mod phase;
mod read;
mod record;
pub mod regs;
mod state;
mod status;
mod write;

pub use config::Config;
pub use record::{RegEvent, Violation};
pub use state::Designware;
