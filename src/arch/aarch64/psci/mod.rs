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

pub mod calls;
pub mod error;
pub mod features;
mod function;
mod method;
mod raw;
mod state;

pub use calls::{affinity_info, cpu_off, cpu_on, cpu_suspend, migrate_info_type};
pub use calls::{system_off, system_reset, system_reset2};
pub use error::PsciError;
pub use features::{features, psci_version, PsciVersion};
pub use method::{set_method, PsciMethod};
pub use raw::{psci_call, psci_call0, psci_call1, psci_call2};
