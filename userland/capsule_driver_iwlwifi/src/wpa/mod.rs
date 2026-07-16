// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The WPA2 security core: the key derivation the four-way handshake rests on.
//! SHA-1, HMAC-SHA1, PBKDF2 and the 802.11i PRF, composed into the pairwise
//! master key and the pairwise transient key. Reached through OP_WPA_PTK. Every
//! primitive is checked against RFC or IEEE known-answer vectors in
//! `iwlwifi_proofs`.

pub mod hmac;
pub mod pbkdf2;
pub mod prf;
pub mod ptk;
pub mod sha1;
pub mod supplicant;
