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

//! What is anchoring the boot measurement on this machine.
//!
//! Four of the six boot gates are pure software and hold on any part: the self
//! test, the Ed25519 and ML-DSA-65 signatures, the transparent STARK, and the
//! handoff. The other two, measured boot and the rollback floor, need somewhere
//! to keep state that software cannot rewrite or rewind. On the machines this
//! bootloader runs on that is a TPM, whether a discrete part or the firmware
//! implementations Intel and AMD expose on the same interface.
//!
//! Naming the anchor rather than assuming it matters because the answer travels
//! with the attestation: a relying party learns what the measurement is worth
//! instead of taking it on faith. Parts anchored some other way get their own
//! value here when the code that reads them exists, and not before.

/// Where this boot's measurements and rollback floor are anchored. The
/// discriminant is mixed into the attestation context, so these values are wire
/// format: add to them, never renumber them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum RootOfTrust {
    /// A TPM 2.0 answered on the command-response interface. Covers a discrete
    /// part on its own bus and the firmware implementations alike, because from
    /// here they are the same protocol with the same guarantee against anything
    /// running after the measurement.
    Tpm = 1,
    /// Nothing is anchoring this boot. Measurements are still computed and the
    /// signatures and the STARK still verify on their own terms, but the floor
    /// can be rewritten by anything privileged enough to reach it.
    Unanchored = 255,
}

impl RootOfTrust {
    /// The byte mixed into the attestation context. Named separately from the
    /// discriminant so the wire format does not move if the enum is reordered.
    #[inline]
    pub fn context_byte(self) -> u8 {
        self as u8
    }

    /// Whether the rollback floor is held somewhere software cannot rewind.
    #[inline]
    pub fn anchors_rollback(self) -> bool {
        matches!(self, RootOfTrust::Tpm)
    }

    /// Whether a production image may boot at this level. An unanchored machine
    /// can run a development build and says so in the log; it cannot claim a
    /// measured boot, so it must not carry a production one.
    #[inline]
    pub fn is_production_grade(self) -> bool {
        self.anchors_rollback()
    }

    /// A short name for the boot log and the attestation report.
    pub fn describe(self) -> &'static str {
        match self {
            RootOfTrust::Tpm => "TPM 2.0",
            RootOfTrust::Unanchored => "none, development only",
        }
    }
}
