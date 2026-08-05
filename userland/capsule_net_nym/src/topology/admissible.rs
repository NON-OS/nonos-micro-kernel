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

//! Whether a directory may be used to build a route.

use super::directory::{DirectoryMeta, Provenance};

/// Whether this directory is admissible, judged by where it came from.
///
/// The three sources do not prove the same thing and are not held to the same
/// test.
///
/// A directory compiled into the image arrived inside a kernel the bootloader
/// measured, verified against two signature schemes and matched to its
/// enrolment before jumping to it. Asking it to also carry an operator
/// signature would mean the capsule refuses to route until somebody installs
/// a key, which is a directory nobody can use.
///
/// A directory that claims to be signed has to prove it, because that claim
/// is the only thing standing behind it.
///
/// A directory fetched over TLS proves who answered, not that an operator
/// vouched for the answer, and that is weaker. It is still admissible,
/// because a route is not trusted on the strength of the list it came from:
/// every hop is sealed to its own packet key, so a hop that is not the node
/// it claimed cannot open its layer. A hostile list therefore costs a dead
/// route rather than a route that looks fine and is not.
pub fn admissible(meta: DirectoryMeta, trusted: impl Fn(&[u8]) -> Option<bool>) -> bool {
    match meta.provenance {
        Provenance::Image => true,
        Provenance::Signed => trusted(&meta.issuer) == Some(true),
        Provenance::Fetched => true,
    }
}
