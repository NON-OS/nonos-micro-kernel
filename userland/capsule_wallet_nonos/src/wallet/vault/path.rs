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

//! Where the blob lives.

/// Under `/data` because that is the tree the block store persists; a file
/// written anywhere else is RAM and gone at the next boot, which would make
/// the whole exercise pointless in a way nobody would notice until they
/// rebooted.
pub const VAULT_PATH: &[u8] = b"/data/wallet.vault";

/// The directory the file needs, created before the file if it is missing.
pub const VAULT_DIR: &[u8] = b"/data";
