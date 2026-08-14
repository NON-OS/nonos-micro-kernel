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

/// Which authority proved a capsule.
///
/// The distinction is never collapsed. A capsule built on this machine and a
/// capsule shipped by the project are both proved, and both run, but they do
/// not mean the same thing to somebody deciding whether to trust the machine.
/// Attestation reports the authority, so a remote party evaluates it rather
/// than being told everything is equally vendor-signed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authority {
    /// The root compiled into the kernel image. Cannot be added to at runtime.
    Vendor,
    /// A key enrolled on this machine by its user, identified by its slot.
    /// Present only in a session where enrolment happened, and never after a
    /// reboot: the machine forgets, like everything else.
    Developer(u8),
}

impl Authority {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Vendor => "vendor",
            Self::Developer(_) => "developer",
        }
    }

    /// True only for the root shipped in the image. Used where a caller must
    /// not accept locally built code, such as a policy that permits only
    /// vendor capsules to hold hardware capabilities.
    pub const fn is_vendor(self) -> bool {
        matches!(self, Self::Vendor)
    }
}
