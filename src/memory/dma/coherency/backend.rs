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

#[cfg(target_arch = "x86_64")]
#[path = "backend_x86_64.rs"]
mod backend_x86_64;
#[cfg(target_arch = "x86_64")]
pub(super) use backend_x86_64::{sync_for_cpu, sync_for_device};

#[cfg(target_arch = "aarch64")]
#[path = "backend_aarch64/mod.rs"]
mod backend_aarch64;
#[cfg(target_arch = "aarch64")]
pub(super) use backend_aarch64::{sync_for_cpu, sync_for_device};
