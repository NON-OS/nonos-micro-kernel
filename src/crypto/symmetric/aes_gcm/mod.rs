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

mod aes128;
mod aes256;
mod gcm;
mod ghash;

pub const TAG_SIZE: usize = 16;
pub const NONCE_SIZE: usize = 12;

pub use aes128::{aes128_gcm_decrypt, aes128_gcm_encrypt, Aes128Gcm};
pub use aes256::{aes256_gcm_decrypt, aes256_gcm_encrypt, Aes256Gcm};
