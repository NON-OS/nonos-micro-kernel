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

use super::CapabilityToken;
use crate::capabilities::types::Capability;

impl CapabilityToken {
    #[inline]
    pub fn is_admin(&self) -> bool {
        self.grants(Capability::Admin)
    }
    #[inline]
    pub fn can_register_service(&self) -> bool {
        self.grants(Capability::RegisterService)
    }
    #[inline]
    pub fn can_device_enum(&self) -> bool {
        self.grants(Capability::DeviceEnum) || self.grants(Capability::Admin)
    }
    #[inline]
    pub fn can_set_time(&self) -> bool {
        self.grants(Capability::TimeSet)
    }
    // Authority to request the kernel spawn another window (a fresh instance of
    // an embedded, attested app capsule).
    #[inline]
    pub fn can_spawn_window(&self) -> bool {
        self.grants(Capability::SpawnWindow) || self.grants(Capability::Admin)
    }
    // Authority to terminate a process the caller does not parent.
    #[inline]
    pub fn can_control_processes(&self) -> bool {
        self.grants(Capability::ProcessControl) || self.grants(Capability::Admin)
    }
}
