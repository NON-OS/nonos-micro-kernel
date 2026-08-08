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

// The trampoline, the identity map and the INIT-SIPI walk are the PC way of
// releasing a core. PSCI needs none of them, so they stay with x86_64 and
// `start` picks whichever applies.
#[cfg(target_arch = "x86_64")]
mod ap_identity;
#[cfg(target_arch = "x86_64")]
mod ap_start;
#[cfg(target_arch = "x86_64")]
mod ap_unit;
#[cfg(target_arch = "x86_64")]
mod boot_inputs;
mod bsp;
#[cfg(target_arch = "x86_64")]
mod stack;
mod start;
#[cfg(target_arch = "x86_64")]
mod time;

pub use bsp::init_bsp;
pub use start::start_aps;
