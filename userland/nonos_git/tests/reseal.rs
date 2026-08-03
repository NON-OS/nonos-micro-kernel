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
//! Making an altered pack well formed again.

use nonos_git::Sha1;

/// Recompute the trailing SHA-1 after changing a pack, so a test that alters
/// one byte on purpose is testing the check it means to and not the trailer.
pub fn reseal(pack: &mut [u8]) {
    let body = pack.len() - 20;
    let sha = Sha1::digest(&pack[..body]);
    pack[body..].copy_from_slice(&sha);
}
