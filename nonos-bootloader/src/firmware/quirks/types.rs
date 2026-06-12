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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuirkFlags(u32);

impl QuirkFlags {
    pub const NONE: Self = Self(0);
    pub const MMAP_UNSTABLE: Self = Self(1 << 0);
    pub const EBS_RETRY_NEEDED: Self = Self(1 << 1);
    pub const GOP_BROKEN: Self = Self(1 << 2);
    pub const TIMER_BROKEN: Self = Self(1 << 3);
    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub fn union(&self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

pub struct FirmwareQuirk {
    pub vendor: &'static str,
    pub flags: QuirkFlags,
}

pub const KNOWN_QUIRKS: &[FirmwareQuirk] = &[
    FirmwareQuirk { vendor: "American Megatrends", flags: QuirkFlags::MMAP_UNSTABLE },
    FirmwareQuirk { vendor: "InsydeH2O", flags: QuirkFlags::EBS_RETRY_NEEDED },
    FirmwareQuirk { vendor: "Phoenix", flags: QuirkFlags::GOP_BROKEN },
];
