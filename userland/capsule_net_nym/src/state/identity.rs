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
/// Generated on first use and never persisted. A fresh identity every boot means a
/// gateway cannot link two runs of the same machine, which is the property the
/// capsule exists to provide; the cost is that bandwidth credit tied to an
/// identity does not survive a reboot. A caller may still install a specific
/// identity when it needs a stable one.
static IDENTITY: Mutex<Option<Identity>> = Mutex::new(None);

#[derive(Clone, Copy)]
pub struct Identity {
    pub seed: [u8; 32],
    pub public: [u8; 32],
}

pub fn set_client_identity(seed: &[u8; 32], public: &[u8; 32]) {
    *IDENTITY.lock() = Some(Identity { seed: *seed, public: *public });
}

/// The current identity, generating one if none has been installed.
pub fn client_identity() -> Option<Identity> {
    let mut slot = IDENTITY.lock();
    if let Some(id) = *slot {
        return Some(id);
    }
    let generated = generate()?;
    *slot = Some(generated);
    Some(generated)
}

fn generate() -> Option<Identity> {
    let mut seed = [0u8; 32];
    crate::crypto::random::fill_random(&mut seed).ok()?;
    let mut public = [0u8; 32];
    if nonos_libc::crypto_ed25519_pubkey(seed.as_ptr(), public.as_mut_ptr()) != 32 {
        return None;
    }
    Some(Identity { seed, public })
}
