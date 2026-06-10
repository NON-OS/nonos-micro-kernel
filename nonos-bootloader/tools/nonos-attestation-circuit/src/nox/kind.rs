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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WorkKind {
    FleetVerification,
    RuntimeBoot,
    HardwareBoot,
    CircuitAudit,
    CapsuleAudit,
    CapsuleBuild,
}

impl WorkKind {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "FLEET_VERIFICATION" => Ok(Self::FleetVerification),
            "RUNTIME_BOOT" => Ok(Self::RuntimeBoot),
            "HARDWARE_BOOT" => Ok(Self::HardwareBoot),
            "CIRCUIT_AUDIT" => Ok(Self::CircuitAudit),
            "CAPSULE_AUDIT" => Ok(Self::CapsuleAudit),
            "CAPSULE_BUILD" => Ok(Self::CapsuleBuild),
            other => Err(format!("unknown contribution kind {other}")),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FleetVerification => "FLEET_VERIFICATION",
            Self::RuntimeBoot => "RUNTIME_BOOT",
            Self::HardwareBoot => "HARDWARE_BOOT",
            Self::CircuitAudit => "CIRCUIT_AUDIT",
            Self::CapsuleAudit => "CAPSULE_AUDIT",
            Self::CapsuleBuild => "CAPSULE_BUILD",
        }
    }
}
