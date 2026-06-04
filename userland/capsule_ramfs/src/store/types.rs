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
use alloc::string::String;
use alloc::vec::Vec;

use super::crypto::{KEY_LEN, NONCE_LEN};

pub(super) struct File {
    pub(super) key: [u8; KEY_LEN],
    pub(super) nonce: [u8; NONCE_LEN],
    pub(super) ciphertext: Vec<u8>,
}

pub struct Store {
    pub(super) files: BTreeMap<String, File>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    NotFound,
    CryptoFailure,
}
