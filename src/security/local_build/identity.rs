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

use crate::crypto::rng::get_random_bytes_secure;
use crate::crypto::zk_kernel::PedersenCommitment;

use super::tree::root_for;

pub struct LocalIdentity {
    pub secret: [u8; 32],
    pub blinding: [u8; 32],
    pub commitment: [u8; 32],
    pub root: [u8; 32],
}

static IDENTITY: Mutex<Option<LocalIdentity>> = Mutex::new(None);

/// Secure rather than best effort: a guessable secret is a tree anyone can
/// mint proofs against.
fn mint() -> Option<LocalIdentity> {
    let secret = get_random_bytes_secure().ok()?;
    let blinding = get_random_bytes_secure().ok()?;
    let commitment = PedersenCommitment::commit(&secret, &blinding).commitment;
    let root = root_for(&commitment);
    Some(LocalIdentity { secret, blinding, commitment, root })
}

/// The root to enrol so this machine will run what it builds. Stable for the
/// life of the boot, so a second build does not invalidate the first consent.
pub fn root() -> Option<[u8; 32]> {
    with_identity(|id| id.root)
}

pub(super) fn with_identity<T>(f: impl FnOnce(&LocalIdentity) -> T) -> Option<T> {
    let mut guard = IDENTITY.lock();
    if guard.is_none() {
        *guard = Some(mint()?);
    }
    guard.as_ref().map(f)
}
