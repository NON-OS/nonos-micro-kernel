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

use crate::wipe::wipe;

/// An extended private key: the secp256k1 secret plus the BIP32 chain code.
/// Both halves are wiped on drop; derivation intermediates never outlive the
/// value that produced them.
pub struct Xprv {
    pub key: [u8; 32],
    pub chain: [u8; 32],
}

impl Drop for Xprv {
    fn drop(&mut self) {
        wipe(&mut self.key);
        wipe(&mut self.chain);
    }
}
