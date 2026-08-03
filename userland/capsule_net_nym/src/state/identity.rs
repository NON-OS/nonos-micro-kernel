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

use spin::Mutex;

/// The client identity the gateway handshake signs with.
///
/// Supplied by whoever configures this capsule rather than generated here: an
/// Ed25519 public key cannot be recovered from a signature, and no syscall
/// derives one from a seed, so the caller passes both or the handshake does
/// not run.
static IDENTITY: Mutex<Option<Identity>> = Mutex::new(None);

#[derive(Clone, Copy)]
pub struct Identity {
    pub seed: [u8; 32],
    pub public: [u8; 32],
}

pub fn set_client_identity(seed: &[u8; 32], public: &[u8; 32]) {
    *IDENTITY.lock() = Some(Identity { seed: *seed, public: *public });
}

pub fn client_identity() -> Option<Identity> {
    *IDENTITY.lock()
}
