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

// Recovery from a BIP39 phrase: the word indices arrive from the wallet, the
// checksum is verified BEFORE any derivation, and only a phrase that passes
// derives m/44'/60'/0'/0/0 and stores the account key. A mistyped phrase is
// rejected outright; it can never produce a plausible-but-wrong account. The
// request buffer holding the words is the runner's, wiped after dispatch;
// every copy made here is wiped before return.

use alloc::vec::Vec;

use nonos_hd::bip39::{words_to_entropy, MAX_WORDS};
use nonos_hd::wipe;

use crate::protocol::{encode_response, Request, EACCES, EINVAL, ENOSPC};
use crate::store::{eth_secret_valid, KeyType, Store, StoreError};

pub fn wallet_recover(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    const HDR: usize = 4 + 8 + 8 + 1;
    let p = req.payload;
    if p.len() < HDR {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let payload_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let caller_pid = match super::super::caller::resolve_caller(payload_pid, sender_pid) {
        Some(pid) => pid,
        None => return encode_response(req.seq, EACCES, &[]),
    };
    let now = u64::from_le_bytes([p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11]]);
    let expires_at = u64::from_le_bytes([p[12], p[13], p[14], p[15], p[16], p[17], p[18], p[19]]);
    let count = p[20] as usize;
    if !matches!(count, 12 | 15 | 18 | 21 | 24) || p.len() != HDR + count * 2 {
        return encode_response(req.seq, EINVAL, &[]);
    }

    let mut words = [0u16; MAX_WORDS];
    for (i, slot) in words.iter_mut().take(count).enumerate() {
        *slot = u16::from_le_bytes([p[HDR + i * 2], p[HDR + i * 2 + 1]]);
    }

    // The checksum gate: reject before deriving anything.
    let valid = match words_to_entropy(&words[..count]) {
        Some(mut entropy) => {
            wipe(&mut entropy.0);
            true
        }
        None => false,
    };
    if !valid {
        wipe_words(&mut words);
        return encode_response(req.seq, EINVAL, &[]);
    }

    let Some(mut key) = super::super::hd::account_key_from_words(&words[..count]) else {
        wipe_words(&mut words);
        return encode_response(req.seq, EINVAL, &[]);
    };
    wipe_words(&mut words);
    if !eth_secret_valid(&key) {
        wipe(&mut key);
        return encode_response(req.seq, EINVAL, &[]);
    }

    let result = store.store(KeyType::Secp256k1Eth, &key, caller_pid, now, expires_at);
    wipe(&mut key);
    match result {
        Ok(id) => encode_response(req.seq, 0, &id.to_le_bytes()),
        Err(StoreError::Full) => encode_response(req.seq, ENOSPC, &[]),
        Err(_) => encode_response(req.seq, EINVAL, &[]),
    }
}

fn wipe_words(words: &mut [u16; MAX_WORDS]) {
    for w in words.iter_mut() {
        // SAFETY: volatile write so the mnemonic wipe is not elided.
        unsafe { core::ptr::write_volatile(w, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
