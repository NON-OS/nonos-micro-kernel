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

//! Shared wire-form ABI for the NONOS marketplace pipeline.
//!
//! Field shape and order match the canonical schema at
//! `abi/marketplace_index.schema.json`. The codec is a length-
//! prefixed binary form, not JSON or CBOR: a marketplace index
//! ships as a single signed blob, and a tight binary parser keeps
//! the userland attack surface narrow.
//!
//! This crate covers the *index* layer only. Per-package detail —
//! the manifest, per-arch artifacts, package signatures — lives
//! inside the package and is governed by
//! `abi/capsule_manifest.schema.json` and
//! `abi/capsule_package.schema.json`. capsule_market parses the
//! index and serves it; capsule_installer (later) pulls the
//! package and walks the manifest. The two layers are
//! intentionally disjoint so the marketplace layer can never speak
//! for the manifest layer.
//!
//! Field-name conventions follow the canonical schemas:
//! - `required_capabilities` holds the `CAP_*` names defined in
//!   `capsule_manifest.schema.json#/$defs/CapName`. The crate
//!   carries them as opaque strings; rejecting an unknown name is
//!   the installer's job.
//! - `supported_arches` holds the canonical target triples from
//!   `capsule_package.schema.json` (`x86_64-nonos`,
//!   `aarch64-nonos`, `riscv64-nonos`).
//! - `PriceModel` mirrors the four payment-policy modes from the
//!   manifest (`free`, `one_time`, `subscription`, `usage_metered`).

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod codec;
pub mod limits;
pub mod types;

pub use codec::{decode_index, release_signing_bytes, DecodeError};
#[cfg(feature = "canonical-encode")]
pub use codec::{encode_and_sign, encode_index, EncodedIndex};
pub use limits::{
    MAX_ARCHES, MAX_CAPABILITIES, MAX_DESCRIPTION, MAX_ENTRIES, MAX_NAME, MAX_PUBLISHER,
    MAX_RELEASES, MAX_SIGNATURE, MAX_SUPPORTED_ARCH_LEN, MAX_TOKEN_SYMBOL, MAX_URL, PUBKEY_LEN,
    SHA256_LEN, SIG_LEN,
};
pub use types::{
    CapsuleRelease, InstallReadiness, MarketplaceEntry, MarketplaceIndex, PriceKind, PriceModel,
    TokenInfo, ValidationReport, ValidationStatus,
};
