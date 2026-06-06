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

//! One concrete release of a marketplace entry. A release is the
//! signed unit the future capsule_installer fetches and verifies;
//! everything an installer needs to refuse a stale, mistargeted, or
//! tampered package lives here.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::validation::ValidationReport;

#[derive(Clone)]
pub struct CapsuleRelease {
    /// Marketplace-assigned release identifier. Stable across
    /// re-publishes of the same artifact bytes.
    pub release_id: String,
    /// Manifest hash (BLAKE3-256 of canonical manifest bytes).
    pub manifest_hash: [u8; 32],
    /// Package hash (BLAKE3-256 of the artifact archive).
    pub package_hash: [u8; 32],
    /// HTTPS URL the installer pulls the package from. Empty when
    /// the entry is index-only (no fetchable artifact).
    pub package_url: String,
    /// Publisher's Ed25519 signature over `release_signing_bytes`.
    /// This covers the artifact hashes, URL, supported arches,
    /// kernel ABI, and requested capabilities. It deliberately does
    /// not cover the marketplace-operator validation report, which
    /// is signed by the enclosing index.
    pub publisher_signature: Vec<u8>,
    /// Architecture triples the release supports (e.g.
    /// "x86_64-nonos"). At least one entry is required.
    pub supported_arches: Vec<String>,
    /// Minimum kernel ABI version this release targets.
    pub kernel_abi_min: u32,
    /// Capability names the manifest requests at install time.
    pub required_capabilities: Vec<String>,
    /// Marketplace operator's validation report.
    pub validation: ValidationReport,
}
