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

//! ChaCha20-Poly1305 AEAD (RFC 8439) for sealing the keyring seed at rest,
//! from scratch and `no_std`. `seal` encrypts a plaintext under a key, nonce
//! and associated data, appending the 16-byte tag; `open` verifies the tag in
//! constant time before decrypting, returning None on any mismatch so a
//! tampered or wrong-key blob never decrypts to usable bytes. The nonce must
//! never repeat under one key; `SealState` enforces that with a monotonic
//! counter so callers cannot reuse one by accident.

#![no_std]

mod chacha20;
mod poly1305;
mod seal;

pub use seal::{open, seal, SealError, SealState, NONCE_LEN, TAG_LEN};

#[cfg(test)]
mod tests;
