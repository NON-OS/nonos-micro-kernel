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

mod capabilities;
mod query;
mod version;

pub use capabilities::PsciCapabilities;
pub use query::{features, has_affinity_info, has_cpu_off, has_cpu_on, has_cpu_suspend, has_mem_protect, has_system_off, has_system_reset, has_system_reset2, has_system_suspend, is_function_supported};
pub use version::{psci_version, PsciVersion};
