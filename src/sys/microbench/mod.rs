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

//! Cycle level measurements of the kernel's own paths.
//!
//! Every case times the real function through the same call the kernel makes,
//! subtracts the cost of the measurement itself, and reports quantiles rather
//! than a mean, because the tail is what a caller waits on.
//!
//! Behind a feature: the loop runs thousands of iterations at boot and has no
//! place in an image anyone ships.

mod counter_overhead;
mod ipc_message;
mod report;
mod run;
mod sample;

pub use run::run;
