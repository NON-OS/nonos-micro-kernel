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

use crate::bip32::{child_hardened, child_normal, compress_pubkey, master_from_seed, Xprv};
use crate::wipe::wipe;

/// Walk the standard Ethereum account path m/44'/60'/0'/0/0 from a BIP39
/// seed and return the account private key. The two non-hardened steps need
/// the parent public key, supplied by `pubkey`: given a 32-byte secret it
/// returns the 65-byte uncompressed SEC1 public key, or None on failure. In
/// the capsule that provider is the kernel `crypto_secp256k1_pubkey`
/// syscall; on the host it is the audited k256 crate. Every intermediate
/// extended key wipes itself; on any failure the output is zeroed.
pub fn derive_eth_key<F>(seed: &[u8; 64], mut pubkey: F, out: &mut [u8; 32]) -> bool
where
    F: FnMut(&[u8; 32]) -> Option<[u8; 65]>,
{
    wipe(out);
    let Some(master) = master_from_seed(seed) else {
        return false;
    };
    let Some(purpose) = child_hardened(&master, 44) else {
        return false;
    };
    let Some(coin) = child_hardened(&purpose, 60) else {
        return false;
    };
    let Some(account) = child_hardened(&coin, 0) else {
        return false;
    };

    let Some(change) = normal_step(&account, &mut pubkey, 0) else {
        return false;
    };
    let Some(address) = normal_step(&change, &mut pubkey, 0) else {
        return false;
    };

    out.copy_from_slice(&address.key);
    true
}

fn normal_step<F>(parent: &Xprv, pubkey: &mut F, index: u32) -> Option<Xprv>
where
    F: FnMut(&[u8; 32]) -> Option<[u8; 65]>,
{
    let mut uncompressed = pubkey(&parent.key)?;
    let compressed = compress_pubkey(&uncompressed);
    wipe(&mut uncompressed);
    child_normal(parent, &compressed?, index)
}
