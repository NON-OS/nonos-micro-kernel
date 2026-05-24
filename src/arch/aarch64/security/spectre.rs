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

mod active;
mod barrier;
mod csv;
mod init;
mod kind;
mod load;
mod ssbs;

pub use active::mitigations_active;
pub use barrier::{clear_prediction_state, speculative_barrier};
pub use csv::{is_csv2_enabled, is_csv3_enabled};
pub use init::init_spectre_mitigations;
pub use kind::SpectreMitigation;
pub use load::speculation_safe_load;
pub use ssbs::{enter_kernel, exit_kernel};
