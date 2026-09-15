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

//! The object attributes every attestation key must carry.
//!
//! `fixedTPM | fixedParent | sensitiveDataOrigin | userWithAuth | restricted
//! | sign`. `restricted` is the one that matters: a restricted signing key
//! will only sign digests the TPM itself produced, so it cannot be used to
//! sign an attestation structure handed to it from outside. Without it a
//! quote proves nothing, because anyone able to talk to the TPM could have
//! it sign a fabricated `TPMS_ATTEST`. The template derives the key with
//! these bits and the parser refuses a public area without them.

pub(super) const OBJECT_ATTRIBUTES: u32 = 0x0005_0072;
