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

//! Reading a NØNOS attestation receipt, from the other side.
//!
//! The machine hands out two things: a document whose weight is a TPM
//! signature over a registry root and a challenge the checker chose, and the
//! capsule entries that root is a digest of. This is the checker's half. It
//! folds the entries as the kernel does, compares against the signed root, and
//! only then reads the entries out.
//!
//! Nothing here trusts the entries until the fold matches, and after it
//! matches nothing needs to. A machine that edited a capability mask produces
//! a set that folds elsewhere, and the signature it is stapled to is over the
//! old one. So the check needs no trust in the sender, only in the TPM key.
//!
//! What that buys: "the program that held your key could not open a socket"
//! stops being a line in documentation and becomes a bitmask inside a signed
//! set, beside the measurement of the binary that held it.

pub mod kernel;

mod cli;
mod decode;
mod entries;
mod hexcode;
mod render;
mod root;

pub use cli::{parse_args, Mode, USAGE};
pub use decode::{capability_names, reaches_network, Authority};
pub use entries::{parse_entries, Entry, ParseError};
pub use hexcode::parse_root;
pub use render::render;
pub use root::{fold_root, verify_root, RootMismatch};

#[cfg(test)]
mod tests;
