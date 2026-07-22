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

// Shared HD-wallet derivation for the generate and recover handlers: mnemonic
// word indices to the m/44'/60'/0'/0/0 account key. The mnemonic, seed and
// every intermediate live only on this stack and are wiped before return; the
// public keys the non-hardened steps need come from the kernel's proven
// secp256k1 via the pubkey syscall.

use nonos_hd::bip39::seed_from_words;
use nonos_hd::{derive_eth_key, wipe};

/// Uncompressed SEC1 public key for a secret, from the kernel syscall. None
/// when the kernel rejects the scalar.
pub(super) fn syscall_pubkey(sk: &[u8; 32]) -> Option<[u8; 65]> {
    let mut out = [0u8; 65];
    if nonos_libc::crypto_secp256k1_pubkey(sk.as_ptr(), out.as_mut_ptr()) == 65 {
        Some(out)
    } else {
        None
    }
}

/// Derive the Ethereum account private key for a BIP39 mnemonic given as
/// wordlist indices, empty passphrase, path m/44'/60'/0'/0/0. The 64-byte
/// seed is wiped before returning. None when derivation fails; the caller
/// treats that as a failed request, never as a partial key.
pub(super) fn account_key_from_words(indices: &[u16]) -> Option<[u8; 32]> {
    let mut seed = [0u8; 64];
    if !seed_from_words(indices, b"", &mut seed) {
        return None;
    }
    let mut key = [0u8; 32];
    let ok = derive_eth_key(&seed, syscall_pubkey, &mut key);
    wipe(&mut seed);
    if ok {
        Some(key)
    } else {
        wipe(&mut key);
        None
    }
}
