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

#[cfg(all(feature = "mldsa2", not(feature = "mldsa3"), not(feature = "mldsa5")))]
extern "C" {
    fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> i32;
    fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_signature(
        sig: *mut u8,
        siglen: *mut usize,
        m: *const u8,
        mlen: usize,
        sk: *const u8,
    ) -> i32;
    fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_verify(
        sig: *const u8,
        siglen: usize,
        m: *const u8,
        mlen: usize,
        pk: *const u8,
    ) -> i32;
}

#[cfg(all(feature = "mldsa2", not(feature = "mldsa3"), not(feature = "mldsa5")))]
pub(super) unsafe fn keypair(pk: *mut u8, sk: *mut u8) -> i32 {
    unsafe { PQCLEAN_MLDSA44_CLEAN_crypto_sign_keypair(pk, sk) }
}
#[cfg(all(feature = "mldsa2", not(feature = "mldsa3"), not(feature = "mldsa5")))]
pub(super) unsafe fn sign(
    sig: *mut u8,
    siglen: *mut usize,
    m: *const u8,
    mlen: usize,
    sk: *const u8,
) -> i32 {
    unsafe { PQCLEAN_MLDSA44_CLEAN_crypto_sign_signature(sig, siglen, m, mlen, sk) }
}
#[cfg(all(feature = "mldsa2", not(feature = "mldsa3"), not(feature = "mldsa5")))]
pub(super) unsafe fn verify(
    sig: *const u8,
    siglen: usize,
    m: *const u8,
    mlen: usize,
    pk: *const u8,
) -> i32 {
    unsafe { PQCLEAN_MLDSA44_CLEAN_crypto_sign_verify(sig, siglen, m, mlen, pk) }
}

#[cfg(all(feature = "mldsa3", not(feature = "mldsa2"), not(feature = "mldsa5")))]
extern "C" {
    fn PQCLEAN_MLDSA65_CLEAN_crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> i32;
    fn PQCLEAN_MLDSA65_CLEAN_crypto_sign_signature(
        sig: *mut u8,
        siglen: *mut usize,
        m: *const u8,
        mlen: usize,
        sk: *const u8,
    ) -> i32;
    fn PQCLEAN_MLDSA65_CLEAN_crypto_sign_verify(
        sig: *const u8,
        siglen: usize,
        m: *const u8,
        mlen: usize,
        pk: *const u8,
    ) -> i32;
}

#[cfg(all(feature = "mldsa3", not(feature = "mldsa2"), not(feature = "mldsa5")))]
pub(super) unsafe fn keypair(pk: *mut u8, sk: *mut u8) -> i32 {
    unsafe { PQCLEAN_MLDSA65_CLEAN_crypto_sign_keypair(pk, sk) }
}
#[cfg(all(feature = "mldsa3", not(feature = "mldsa2"), not(feature = "mldsa5")))]
pub(super) unsafe fn sign(
    sig: *mut u8,
    siglen: *mut usize,
    m: *const u8,
    mlen: usize,
    sk: *const u8,
) -> i32 {
    unsafe { PQCLEAN_MLDSA65_CLEAN_crypto_sign_signature(sig, siglen, m, mlen, sk) }
}
#[cfg(all(feature = "mldsa3", not(feature = "mldsa2"), not(feature = "mldsa5")))]
pub(super) unsafe fn verify(
    sig: *const u8,
    siglen: usize,
    m: *const u8,
    mlen: usize,
    pk: *const u8,
) -> i32 {
    unsafe { PQCLEAN_MLDSA65_CLEAN_crypto_sign_verify(sig, siglen, m, mlen, pk) }
}

#[cfg(all(feature = "mldsa5", not(feature = "mldsa2"), not(feature = "mldsa3")))]
extern "C" {
    fn PQCLEAN_MLDSA87_CLEAN_crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> i32;
    fn PQCLEAN_MLDSA87_CLEAN_crypto_sign_signature(
        sig: *mut u8,
        siglen: *mut usize,
        m: *const u8,
        mlen: usize,
        sk: *const u8,
    ) -> i32;
    fn PQCLEAN_MLDSA87_CLEAN_crypto_sign_verify(
        sig: *const u8,
        siglen: usize,
        m: *const u8,
        mlen: usize,
        pk: *const u8,
    ) -> i32;
}

#[cfg(all(feature = "mldsa5", not(feature = "mldsa2"), not(feature = "mldsa3")))]
pub(super) unsafe fn keypair(pk: *mut u8, sk: *mut u8) -> i32 {
    unsafe { PQCLEAN_MLDSA87_CLEAN_crypto_sign_keypair(pk, sk) }
}
#[cfg(all(feature = "mldsa5", not(feature = "mldsa2"), not(feature = "mldsa3")))]
pub(super) unsafe fn sign(
    sig: *mut u8,
    siglen: *mut usize,
    m: *const u8,
    mlen: usize,
    sk: *const u8,
) -> i32 {
    unsafe { PQCLEAN_MLDSA87_CLEAN_crypto_sign_signature(sig, siglen, m, mlen, sk) }
}
#[cfg(all(feature = "mldsa5", not(feature = "mldsa2"), not(feature = "mldsa3")))]
pub(super) unsafe fn verify(
    sig: *const u8,
    siglen: usize,
    m: *const u8,
    mlen: usize,
    pk: *const u8,
) -> i32 {
    unsafe { PQCLEAN_MLDSA87_CLEAN_crypto_sign_verify(sig, siglen, m, mlen, pk) }
}

#[cfg(all(not(feature = "mldsa2"), not(feature = "mldsa3"), not(feature = "mldsa5")))]
extern "C" {
    fn PQCLEAN_MLDSA65_CLEAN_crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> i32;
    fn PQCLEAN_MLDSA65_CLEAN_crypto_sign_signature(
        sig: *mut u8,
        siglen: *mut usize,
        m: *const u8,
        mlen: usize,
        sk: *const u8,
    ) -> i32;
    fn PQCLEAN_MLDSA65_CLEAN_crypto_sign_verify(
        sig: *const u8,
        siglen: usize,
        m: *const u8,
        mlen: usize,
        pk: *const u8,
    ) -> i32;
}

#[cfg(all(not(feature = "mldsa2"), not(feature = "mldsa3"), not(feature = "mldsa5")))]
pub(super) unsafe fn keypair(pk: *mut u8, sk: *mut u8) -> i32 {
    unsafe { PQCLEAN_MLDSA65_CLEAN_crypto_sign_keypair(pk, sk) }
}
#[cfg(all(not(feature = "mldsa2"), not(feature = "mldsa3"), not(feature = "mldsa5")))]
pub(super) unsafe fn sign(
    sig: *mut u8,
    siglen: *mut usize,
    m: *const u8,
    mlen: usize,
    sk: *const u8,
) -> i32 {
    unsafe { PQCLEAN_MLDSA65_CLEAN_crypto_sign_signature(sig, siglen, m, mlen, sk) }
}
#[cfg(all(not(feature = "mldsa2"), not(feature = "mldsa3"), not(feature = "mldsa5")))]
pub(super) unsafe fn verify(
    sig: *const u8,
    siglen: usize,
    m: *const u8,
    mlen: usize,
    pk: *const u8,
) -> i32 {
    unsafe { PQCLEAN_MLDSA65_CLEAN_crypto_sign_verify(sig, siglen, m, mlen, pk) }
}
