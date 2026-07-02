// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Blake3_256 = 1,
}

impl HashAlgorithm {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Blake3_256),
            _ => None,
        }
    }

    pub const fn hash_size(&self) -> usize {
        match self {
            Self::Blake3_256 => 32,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureAlgorithm {
    Ed25519 = 1,
    Ed25519MlDsa65 = 2,
}

impl SignatureAlgorithm {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Ed25519),
            2 => Some(Self::Ed25519MlDsa65),
            _ => None,
        }
    }

    pub const fn signature_size(&self) -> usize {
        match self {
            Self::Ed25519 => 64,
            Self::Ed25519MlDsa65 => 8 + 32 + 64 + 32 + 3309,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Production = 1,
}

impl ImageFormat {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Production),
            _ => None,
        }
    }
}

pub mod flags {
    pub const HAS_ZK_PROOF: u32 = 1 << 0;
    pub const HARDENED_MODE: u32 = 1 << 1;
    pub const DEVELOPER_MODE: u32 = 1 << 2;
}
