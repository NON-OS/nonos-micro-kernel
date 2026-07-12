// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The WPA2 four-way handshake message layer: parsing EAPOL-Key frames and
//! verifying their MIC under the KCK derived from the pairwise transient key.
//! Reached through OP_EAPOL_VERIFY. The MIC rests on the RFC-verified HMAC-SHA1.

pub mod mic;
pub mod parse;
