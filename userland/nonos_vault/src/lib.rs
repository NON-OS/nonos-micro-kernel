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

//! Sealing something to this machine, so it can be written anywhere.
//!
//! A NONOS machine is amnesic on purpose: the filesystem starts empty and
//! nothing on disk is trusted. That is right for a privacy system and wrong
//! for anything a person uses twice, so there has to be one way to keep a
//! thing that is safe to keep. This is it.
//!
//! A sealed record is ChaCha20-Poly1305 over the caller's bytes, under a key
//! the TPM derives from its owner seed and the boot PCRs. The blob can go on a
//! disk somebody steals: it opens on this machine, running this kernel, and
//! nowhere else. There is no passphrase to phish and no key file to copy.
//!
//! Each record gets its own key, derived from the machine root through HKDF
//! with the record's name. Recovering the key for one record does not open
//! another. That matters more as the number of records grows, which is the
//! direction this is going: the wallet was first, and settings, window
//! layout and per-capsule storage are the same shape.
//!
//! What this deliberately does not do:
//!
//! It does not survive a firmware or kernel change. Moving a bound PCR
//! changes the root and every record stops opening, which is the correct
//! outcome for a seal that means "this machine in this state" and the reason
//! a wallet still needs its phrase written down.
//!
//! It does not stop rollback. Someone holding the disk can put back an older
//! blob, and the seal proves the machine rather than the version. Fixing that
//! needs a monotonic counter in TPM NV storage, which this kernel's TPM stack
//! does not yet drive.
//!
//! It is not a place to keep something ephemeral. A capsule whose whole point
//! is to be unlinkable between boots, like the mixnet client's identity,
//! should keep generating a fresh one; sealing it would quietly remove the
//! property it exists to provide.

#![no_std]

extern crate alloc;

mod blob;
mod derive;
mod error;
mod hmac;
mod open;
mod parse;
mod seal;
mod seal_body;
mod subkey;
mod wipe;

pub use blob::{HEADER_LEN, OVERHEAD};
pub use error::VaultError;
pub use open::open;
pub use seal::seal;
pub use derive::subkey;
pub use subkey::{KeyError, MAX_RECORD, ROOT_LABEL};

#[cfg(test)]
mod tests;
