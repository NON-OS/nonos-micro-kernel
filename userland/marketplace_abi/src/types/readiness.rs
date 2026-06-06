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

//! Verdict the capsule emits when a caller asks "is this release
//! ready to install?". Five independent gates must all pass; the
//! report carries which ones tripped so a UI can explain the
//! refusal precisely.

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InstallReadiness {
    /// Final answer: install allowed iff all five flags are true.
    pub install_ready: bool,
    /// `index_signature` verifies against the operator pubkey.
    pub index_signature_valid: bool,
    /// `package_url` is non-empty.
    pub package_url_present: bool,
    /// Publisher signature verifies against the listing pubkey.
    /// The field name is kept for wire compatibility with earlier
    /// six-byte readiness replies.
    pub publisher_signature_present: bool,
    /// Operator's `validation_status` is `Validated`.
    pub validation_passed: bool,
    /// Running kernel arch is in the release's `supported_arches`.
    pub arch_match: bool,
}

impl InstallReadiness {
    pub fn refused() -> Self {
        Self {
            install_ready: false,
            index_signature_valid: false,
            package_url_present: false,
            publisher_signature_present: false,
            validation_passed: false,
            arch_match: false,
        }
    }

    /// Compose a verdict from the five checks. `install_ready` is
    /// the AND of the inputs; anything `false` blocks install.
    pub fn from_checks(
        index_signature_valid: bool,
        package_url_present: bool,
        publisher_signature_verified: bool,
        validation_passed: bool,
        arch_match: bool,
    ) -> Self {
        let install_ready = index_signature_valid
            && package_url_present
            && publisher_signature_verified
            && validation_passed
            && arch_match;
        Self {
            install_ready,
            index_signature_valid,
            package_url_present,
            publisher_signature_present: publisher_signature_verified,
            validation_passed,
            arch_match,
        }
    }
}
