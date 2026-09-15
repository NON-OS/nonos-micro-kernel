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

//! A wallet that is still there after a reboot.
//!
//! Everything about this system is amnesic: the filesystem starts empty, the
//! keyring starts empty, and nothing on disk is trusted. That is the right
//! default for a privacy machine and the wrong one for a wallet, which until
//! now had to be recovered from its twelve words on every single boot.
//!
//! The keyring seals the account key to this machine's TPM, bound to the boot
//! state, and hands back a blob that means nothing anywhere else. This module
//! puts that blob on the disk and brings it back. The wallet never sees the
//! key: it asks for a blob, stores bytes, and later hands the same bytes back
//! for the keyring to open.
//!
//! If the blob will not open, because the machine changed or the disk was
//! moved, the wallet is where it was before: the mnemonic is the way in, and
//! the one-time backup screen is still the moment that matters.

mod answer;
mod format;
mod keyring;
mod load;
mod load_ops;
mod load_read;
mod path;
mod persist;
mod remember;
mod save;
mod save_ops;
mod vfs;

pub use remember::{recall, remember, Recall};
