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

// A 128-bit FNV-1a fold over an entry's raw payload, recomputed at read time to
// catch a corrupt or bit-flipped sector before it is spawned. This is a
// tamper-evidence check under the publisher signature, not the trust root, so it
// stays self-contained here: the crypto_hash syscall needs Capability::Crypto,
// which this capsule deliberately does not hold. tools/nonos-store-pack folds the
// same bytes the same way and stores the result in each TOC entry's reserved
// tail; both sides must stay byte-identical.
const FNV_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
const FNV_PRIME: u128 = 0x0000000001000000000000000000013b;

pub fn digest16(data: &[u8]) -> [u8; 16] {
    let mut hash = FNV_OFFSET;
    for &byte in data {
        hash = (hash ^ byte as u128).wrapping_mul(FNV_PRIME);
    }
    hash.to_be_bytes()
}
