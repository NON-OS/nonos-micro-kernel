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

pub(super) const PSCI_VERSION: u32 = 0x8400_0000;
pub(super) const PSCI_CPU_SUSPEND_64: u32 = 0xC400_0001;
pub(super) const PSCI_CPU_OFF: u32 = 0x8400_0002;
pub(super) const PSCI_CPU_ON_64: u32 = 0xC400_0003;
pub(super) const PSCI_AFFINITY_INFO_64: u32 = 0xC400_0004;
pub(super) const PSCI_MIGRATE_INFO_TYPE: u32 = 0x8400_0006;
pub(super) const PSCI_SYSTEM_OFF: u32 = 0x8400_0008;
pub(super) const PSCI_SYSTEM_RESET: u32 = 0x8400_0009;
pub(super) const PSCI_FEATURES: u32 = 0x8400_000A;
pub(super) const PSCI_CPU_DEFAULT_SUSPEND_64: u32 = 0xC400_000C;
pub(super) const PSCI_NODE_HW_STATE_64: u32 = 0xC400_000D;
pub(super) const PSCI_SYSTEM_SUSPEND_64: u32 = 0xC400_000E;
pub(super) const PSCI_SYSTEM_RESET2_64: u32 = 0xC400_0012;
pub(super) const PSCI_MEM_PROTECT: u32 = 0x8400_0013;
