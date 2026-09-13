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

//! Who signed the capsule.
//!
//! The kernel's own `Authority` cannot be included here: it sits inside the
//! dev-root module and pulls in that module's storage. Only the byte encoding
//! is restated, and `authority_tests` pins it.

/// Developer slots in the kernel this reader was built against.
const MAX_DEV_ROOTS: u8 = 4;
/// The kernel's byte for a publisher-signed, unproved capsule.
const PUBLISHER: u8 = 255;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Authority {
    /// The root compiled into the kernel image.
    Vendor,
    /// A key the machine's owner enrolled, by slot.
    Developer(u8),
    /// A third-party publisher's signature, with no proof behind it.
    Publisher,
}

impl Authority {
    /// Inverts the kernel's `Vendor => 0, Publisher => 255, Developer(s) => 1 + s`.
    pub fn from_byte(b: u8) -> Self {
        match b {
            0 => Self::Vendor,
            PUBLISHER => Self::Publisher,
            n => Self::Developer(n - 1),
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Self::Vendor => "vendor".to_string(),
            Self::Publisher => "publisher signature, no proof".to_string(),
            /*
             * A slot past this reader's table still gets reported. The byte
             * folded into a signed root, so it is authentic, and it means the
             * machine runs a kernel with more slots than this build knows.
             */
            Self::Developer(s) if *s >= MAX_DEV_ROOTS => {
                format!("developer key {s}, a slot beyond this reader's table")
            }
            Self::Developer(s) => format!("developer key {s}"),
        }
    }
}
