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
#[repr(u8)]
pub enum CpuVendor {
    Unknown = 0,
    Intel = 1,
    Amd = 2,
    Via = 3,
    Transmeta = 4,
    Cyrix = 5,
    Centaur = 6,
    Hygon = 7,
}

impl CpuVendor {
    pub fn from_cpuid_string(ebx: u32, ecx: u32, edx: u32) -> Self {
        if ebx == 0x756e6547 && edx == 0x49656e69 && ecx == 0x6c65746e {
            Self::Intel
        } else if ebx == 0x68747541 && edx == 0x69746e65 && ecx == 0x444d4163 {
            Self::Amd
        } else if ebx == 0x746e6543 && edx == 0x48727561 && ecx == 0x736c7561 {
            Self::Centaur
        } else if ebx == 0x20414956 && edx == 0x20414956 && ecx == 0x20414956 {
            Self::Via
        } else if ebx == 0x756e6547 && edx == 0x4d656e69 && ecx == 0x36387854 {
            Self::Transmeta
        } else if ebx == 0x69727943 && edx == 0x736e4978 && ecx == 0x64616574 {
            Self::Cyrix
        } else if ebx == 0x6f677948 && edx == 0x6e65476e && ecx == 0x656e6975 {
            Self::Hygon
        } else {
            Self::Unknown
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Intel => "Intel",
            Self::Amd => "AMD",
            Self::Via => "VIA",
            Self::Transmeta => "Transmeta",
            Self::Cyrix => "Cyrix",
            Self::Centaur => "Centaur",
            Self::Hygon => "Hygon",
        }
    }
}
