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

// HD wallet generation: hardware entropy becomes a 12-word BIP39 mnemonic,
// the mnemonic becomes the m/44'/60'/0'/0/0 account key, and the key is
// stored under the caller's pid. The response carries the wallet id and the
// word indices ONCE, for the one-time backup screen; the mnemonic is not
// retained here in any form, so this response is the only chance to write it
// down. Everything secret on this path is stack-local and wiped.

use alloc::vec::Vec;

use nonos_hd::bip39::{entropy_to_words, MAX_WORDS};
use nonos_hd::wipe;

use crate::protocol::{encode_response, Request, EACCES, EINVAL, ENOSPC};
use crate::store::{eth_secret_valid, KeyType, Store, StoreError};

const WORDS: usize = 12;

pub fn wallet_generate_hd(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    const HDR: usize = 4 + 8 + 8;
    if req.payload.len() != HDR {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let payload_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let caller_pid = match super::super::caller::resolve_caller(payload_pid, sender_pid) {
        Some(pid) => pid,
        None => return encode_response(req.seq, EACCES, &[]),
    };
    let now = u64::from_le_bytes([p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11]]);
    let expires_at = u64::from_le_bytes([p[12], p[13], p[14], p[15], p[16], p[17], p[18], p[19]]);

    let mut words = [0u16; MAX_WORDS];
    let mut key = [0u8; 32];
    let mut ok = false;
    for attempt in 0..64 {
        // 128-bit entropy from the mixed hardware sources; the checksum word
        // is computed by the crate, so the phrase always validates on entry.
        let mut entropy32 = [0u8; 32];
        if !crate::entropy::gather_secret(&mut entropy32) {
            if attempt & 3 == 3 {
                for _ in 0..1000 {
                    nonos_libc::mk_yield();
                }
            }
            continue;
        }
        let mut entropy = [0u8; 16];
        entropy.copy_from_slice(&entropy32[..16]);
        wipe(&mut entropy32);

        let derived = entropy_to_words(&entropy, &mut words) == Some(WORDS);
        wipe(&mut entropy);
        if !derived {
            continue;
        }
        match super::super::hd::account_key_from_words(&words[..WORDS]) {
            Some(k) if eth_secret_valid(&k) => {
                key = k;
                ok = true;
                break;
            }
            _ => {
                wipe_words(&mut words);
                continue;
            }
        }
    }
    if !ok {
        wipe_words(&mut words);
        wipe(&mut key);
        return encode_response(req.seq, EINVAL, &[]);
    }

    let result = store.store(KeyType::Secp256k1Eth, &key, caller_pid, now, expires_at);
    wipe(&mut key);

    let resp = match result {
        Ok(id) => {
            let mut payload = [0u8; 4 + 1 + WORDS * 2];
            payload[..4].copy_from_slice(&id.to_le_bytes());
            payload[4] = WORDS as u8;
            for (i, &w) in words.iter().take(WORDS).enumerate() {
                payload[5 + i * 2..7 + i * 2].copy_from_slice(&w.to_le_bytes());
            }
            let out = encode_response(req.seq, 0, &payload);
            wipe(&mut payload);
            out
        }
        Err(StoreError::Full) => encode_response(req.seq, ENOSPC, &[]),
        Err(_) => encode_response(req.seq, EINVAL, &[]),
    };
    wipe_words(&mut words);
    resp
}

fn wipe_words(words: &mut [u16; MAX_WORDS]) {
    for w in words.iter_mut() {
        // SAFETY: volatile write so the mnemonic wipe is not elided.
        unsafe { core::ptr::write_volatile(w, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
