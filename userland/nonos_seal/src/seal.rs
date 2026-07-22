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

use core::sync::atomic::{compiler_fence, Ordering};

use crate::chacha20;
use crate::poly1305::Poly1305;

pub const NONCE_LEN: usize = 12;
pub const TAG_LEN: usize = 16;

#[derive(Debug, PartialEq, Eq)]
pub enum SealError {
    /// The nonce space for this key is exhausted; the caller must rekey rather
    /// than ever repeat a nonce.
    NonceExhausted,
    /// The authentication tag did not verify: wrong key, tampered ciphertext,
    /// or wrong associated data.
    AuthFailed,
    /// The provided buffer was too short to hold ciphertext plus tag.
    ShortBuffer,
}

/// Derive the Poly1305 one-time key: ChaCha20 block 0, first 32 bytes.
fn poly_key(key: &[u8; 32], nonce: &[u8; NONCE_LEN]) -> [u8; 32] {
    let block = chacha20::block(key, nonce, 0);
    let mut out = [0u8; 32];
    out.copy_from_slice(&block[..32]);
    out
}

/// Compute the AEAD tag over associated data and ciphertext, per RFC 8439:
/// aad, zero pad to 16, ciphertext, zero pad to 16, then the two little-endian
/// 64-bit lengths.
fn tag(otk: &[u8; 32], aad: &[u8], ciphertext: &[u8]) -> [u8; 16] {
    let mut mac = Poly1305::new(otk);
    mac.update(aad);
    pad16(&mut mac, aad.len());
    mac.update(ciphertext);
    pad16(&mut mac, ciphertext.len());
    let mut lengths = [0u8; 16];
    lengths[..8].copy_from_slice(&(aad.len() as u64).to_le_bytes());
    lengths[8..].copy_from_slice(&(ciphertext.len() as u64).to_le_bytes());
    mac.update(&lengths);
    mac.finalize()
}

fn pad16(mac: &mut Poly1305, len: usize) {
    let rem = len % 16;
    if rem != 0 {
        let zeros = [0u8; 16];
        mac.update(&zeros[..16 - rem]);
    }
}

/// Seal `plaintext` into `out`, which must be `plaintext.len() + TAG_LEN`
/// bytes. Writes ciphertext followed by the 16-byte tag. The nonce must be
/// unique for `key`; prefer `SealState` to guarantee that.
pub fn seal(
    key: &[u8; 32],
    nonce: &[u8; NONCE_LEN],
    aad: &[u8],
    plaintext: &[u8],
    out: &mut [u8],
) -> Result<usize, SealError> {
    let total = plaintext.len() + TAG_LEN;
    if out.len() < total {
        return Err(SealError::ShortBuffer);
    }
    let ct = &mut out[..plaintext.len()];
    ct.copy_from_slice(plaintext);
    // Keystream starts at counter 1; counter 0 made the Poly1305 key.
    chacha20::apply_keystream(key, nonce, 1, ct);

    let otk = poly_key(key, nonce);
    let t = tag(&otk, aad, &out[..plaintext.len()]);
    out[plaintext.len()..total].copy_from_slice(&t);
    Ok(total)
}

/// Open a sealed buffer (ciphertext followed by tag) into `out`, which must be
/// at least `sealed.len() - TAG_LEN` bytes. Verifies the tag in constant time
/// first; on any mismatch nothing is written to `out` and AuthFailed is
/// returned, so a wrong key or tampered blob never yields plaintext.
pub fn open(
    key: &[u8; 32],
    nonce: &[u8; NONCE_LEN],
    aad: &[u8],
    sealed: &[u8],
    out: &mut [u8],
) -> Result<usize, SealError> {
    if sealed.len() < TAG_LEN {
        return Err(SealError::AuthFailed);
    }
    let ct_len = sealed.len() - TAG_LEN;
    if out.len() < ct_len {
        return Err(SealError::ShortBuffer);
    }
    let ciphertext = &sealed[..ct_len];
    let received = &sealed[ct_len..];

    let otk = poly_key(key, nonce);
    let expected = tag(&otk, aad, ciphertext);
    if !ct_eq(&expected, received) {
        return Err(SealError::AuthFailed);
    }

    out[..ct_len].copy_from_slice(ciphertext);
    chacha20::apply_keystream(key, nonce, 1, &mut out[..ct_len]);
    Ok(ct_len)
}

/// Constant-time 16-byte tag comparison: never short-circuits, so tag
/// verification does not leak how many bytes matched.
fn ct_eq(a: &[u8; 16], b: &[u8]) -> bool {
    if b.len() != 16 {
        return false;
    }
    let mut diff = 0u8;
    for i in 0..16 {
        diff |= a[i] ^ b[i];
    }
    compiler_fence(Ordering::SeqCst);
    diff == 0
}

/// A per-key nonce sequencer. The 96-bit nonce is a monotonic counter, so no
/// two seals under one key ever share a nonce, the property Poly1305 security
/// depends on. Persist `next` alongside the ciphertext; on reload, continue
/// from it so a nonce is never replayed across reboots.
pub struct SealState {
    key: [u8; 32],
    next: u64,
}

impl SealState {
    pub fn new(key: [u8; 32], next: u64) -> Self {
        Self { key, next }
    }

    /// The next counter value that will be used, for persisting alongside the
    /// blob so nonces do not repeat after a reboot.
    pub fn counter(&self) -> u64 {
        self.next
    }

    /// Seal with the next fresh nonce, advancing the counter. Fails closed if
    /// the counter would wrap, rather than reuse a nonce.
    pub fn seal_next(
        &mut self,
        aad: &[u8],
        plaintext: &[u8],
        out: &mut [u8],
    ) -> Result<(u64, usize), SealError> {
        let counter = self.next;
        let mut nonce = [0u8; NONCE_LEN];
        nonce[4..].copy_from_slice(&counter.to_le_bytes());
        let n = seal(&self.key, &nonce, aad, plaintext, out)?;
        self.next = counter.checked_add(1).ok_or(SealError::NonceExhausted)?;
        Ok((counter, n))
    }

    /// Open a blob sealed at `counter` by this key.
    pub fn open_at(
        &self,
        counter: u64,
        aad: &[u8],
        sealed: &[u8],
        out: &mut [u8],
    ) -> Result<usize, SealError> {
        let mut nonce = [0u8; NONCE_LEN];
        nonce[4..].copy_from_slice(&counter.to_le_bytes());
        open(&self.key, &nonce, aad, sealed, out)
    }
}

impl Drop for SealState {
    fn drop(&mut self) {
        for b in self.key.iter_mut() {
            // SAFETY: volatile write so the key wipe is not optimized out.
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        compiler_fence(Ordering::SeqCst);
    }
}
