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

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const MAX_KEY_SIZE: usize = 256;
pub const MAX_KEYS: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyType {
    Symmetric = 0,
    PrivateKey = 1,
    PublicKey = 2,
    HmacSecret = 3,
    DerivedKey = 4,
    SessionKey = 5,
    MasterKey = 6,
    SigningKey = 7,
    Secp256k1Eth = 8,
}

impl KeyType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Symmetric),
            1 => Some(Self::PrivateKey),
            2 => Some(Self::PublicKey),
            3 => Some(Self::HmacSecret),
            4 => Some(Self::DerivedKey),
            5 => Some(Self::SessionKey),
            6 => Some(Self::MasterKey),
            7 => Some(Self::SigningKey),
            8 => Some(Self::Secp256k1Eth),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

pub(super) struct KeyEntry {
    pub(super) key_type: KeyType,
    pub(super) data: Vec<u8>,
    pub(super) owner_pid: u32,
    pub(super) created_at: u64,
    pub(super) expires_at: u64,
    pub(super) use_count: u64,
    pub(super) locked: bool,
}

pub struct KeyMetadata {
    pub id: u32,
    pub key_type: KeyType,
    pub size: u16,
    pub owner_pid: u32,
    pub created_at: u64,
    pub expires_at: u64,
    pub use_count: u64,
    pub locked: bool,
}

pub struct Store {
    pub(super) entries: BTreeMap<u32, KeyEntry>,
    pub(super) next_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    NotFound,
    AccessDenied,
    Locked,
    Full,
    InvalidArgument,
}
