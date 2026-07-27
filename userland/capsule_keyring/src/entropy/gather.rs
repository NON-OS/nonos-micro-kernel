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

use super::rdrand::fill_hardware;

/// Gather 32 bytes of key material by combining every hardware source the
/// keyring can reach, so no single source has to be trusted alone and a stall
/// in one never fails generation:
///
///   secret = kernel_pool XOR on_die_drbg
///
/// The kernel entropy pool (capability-gated, mixed from the system's sources)
/// is preferred, and the CPU's own RDRAND/RDSEED is XORed in. XOR means the
/// result is at least as strong as the stronger input: an attacker must break
/// or predict BOTH the kernel pool and the on-die DRBG to recover the key. If
/// exactly one source is available the secret still carries its full entropy;
/// only when BOTH are dead does the draw fail.
///
/// Returns true and fills `secret` on success. On failure `secret` is zeroed.
pub fn gather_secret(secret: &mut [u8; 32]) -> bool {
    let mut pool = [0u8; 32];
    let have_pool = nonos_libc::crypto_random(pool.as_mut_ptr(), 32) == 32;

    let mut hw = [0u8; 32];
    let have_hw = fill_hardware(&mut hw);

    let ok = match (have_pool, have_hw) {
        (true, true) => {
            for i in 0..32 {
                secret[i] = pool[i] ^ hw[i];
            }
            true
        }
        (true, false) => {
            secret.copy_from_slice(&pool);
            true
        }
        (false, true) => {
            secret.copy_from_slice(&hw);
            true
        }
        (false, false) => false,
    };

    wipe(&mut pool);
    wipe(&mut hw);
    if !ok {
        wipe(secret);
    }
    ok
}

fn wipe(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
}
