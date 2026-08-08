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

extern crate alloc;
use alloc::vec::Vec;
use core::ptr;

// PQClean C-side glue (`nonos_malloc`, `nonos_free`, `nonos_panic`,
// `nonos_randombytes`) lives in `crate::crypto::pqclean_support`. It
// must stay outside this feature gate — `build.rs` always compiles
// `libc_glue.c` and `randombytes.c` into the kernel rlib, even on
// builds with no PQClean Rust front-end enabled.

#[cfg(feature = "mlkem512")]
pub const KYBER_PARAM_NAME: &str = "ML-KEM-512";
#[cfg(any(feature = "mlkem768", all(not(feature = "mlkem512"), not(feature = "mlkem1024"))))]
pub const KYBER_PARAM_NAME: &str = "ML-KEM-768";
#[cfg(feature = "mlkem1024")]
pub const KYBER_PARAM_NAME: &str = "ML-KEM-1024";

#[cfg(feature = "mlkem512")]
pub const PUBLICKEY_BYTES: usize = 800;
#[cfg(feature = "mlkem512")]
pub const SECRETKEY_BYTES: usize = 1632;
#[cfg(feature = "mlkem512")]
pub const CIPHERTEXT_BYTES: usize = 768;

#[cfg(any(feature = "mlkem768", all(not(feature = "mlkem512"), not(feature = "mlkem1024"))))]
pub const PUBLICKEY_BYTES: usize = 1184;
#[cfg(any(feature = "mlkem768", all(not(feature = "mlkem512"), not(feature = "mlkem1024"))))]
pub const SECRETKEY_BYTES: usize = 2400;
#[cfg(any(feature = "mlkem768", all(not(feature = "mlkem512"), not(feature = "mlkem1024"))))]
pub const CIPHERTEXT_BYTES: usize = 1088;

#[cfg(feature = "mlkem1024")]
pub const PUBLICKEY_BYTES: usize = 1568;
#[cfg(feature = "mlkem1024")]
pub const SECRETKEY_BYTES: usize = 3168;
#[cfg(feature = "mlkem1024")]
pub const CIPHERTEXT_BYTES: usize = 1568;

pub const SHAREDSECRET_BYTES: usize = 32;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct KyberPublicKey {
    pub bytes: [u8; PUBLICKEY_BYTES],
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct KyberSecretKey {
    pub bytes: [u8; SECRETKEY_BYTES],
}

impl Drop for KyberSecretKey {
    fn drop(&mut self) {
        for b in &mut self.bytes {
            unsafe { ptr::write_volatile(b, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

#[repr(C)]
pub struct KyberCiphertext {
    pub bytes: [u8; CIPHERTEXT_BYTES],
}

#[repr(C)]
#[derive(Debug)]
pub struct KyberKeyPair {
    pub public_key: KyberPublicKey,
    pub secret_key: KyberSecretKey,
}

#[cfg(all(feature = "mlkem768", not(feature = "mlkem512"), not(feature = "mlkem1024")))]
mod ffi {
    extern "C" {
        pub(super) fn PQCLEAN_MLKEM768_CLEAN_crypto_kem_keypair(pk: *mut u8, sk: *mut u8) -> i32;
        pub(super) fn PQCLEAN_MLKEM768_CLEAN_crypto_kem_enc(
            ct: *mut u8,
            ss: *mut u8,
            pk: *const u8,
        ) -> i32;
        pub(super) fn PQCLEAN_MLKEM768_CLEAN_crypto_kem_dec(
            ss: *mut u8,
            ct: *const u8,
            sk: *const u8,
        ) -> i32;
    }
    pub(super) unsafe fn keypair(pk: *mut u8, sk: *mut u8) -> i32 {
        unsafe { PQCLEAN_MLKEM768_CLEAN_crypto_kem_keypair(pk, sk) }
    }
    pub(super) unsafe fn encaps(ct: *mut u8, ss: *mut u8, pk: *const u8) -> i32 {
        unsafe { PQCLEAN_MLKEM768_CLEAN_crypto_kem_enc(ct, ss, pk) }
    }
    pub(super) unsafe fn decaps(ss: *mut u8, ct: *const u8, sk: *const u8) -> i32 {
        unsafe { PQCLEAN_MLKEM768_CLEAN_crypto_kem_dec(ss, ct, sk) }
    }
}

#[cfg(feature = "mlkem512")]
mod ffi {
    extern "C" {
        pub fn PQCLEAN_MLKEM512_CLEAN_crypto_kem_keypair(pk: *mut u8, sk: *mut u8) -> i32;
        pub fn PQCLEAN_MLKEM512_CLEAN_crypto_kem_enc(
            ct: *mut u8,
            ss: *mut u8,
            pk: *const u8,
        ) -> i32;
        pub fn PQCLEAN_MLKEM512_CLEAN_crypto_kem_dec(
            ss: *mut u8,
            ct: *const u8,
            sk: *const u8,
        ) -> i32;
    }
    pub unsafe fn keypair(pk: *mut u8, sk: *mut u8) -> i32 {
        unsafe { PQCLEAN_MLKEM512_CLEAN_crypto_kem_keypair(pk, sk) }
    }
    pub unsafe fn encaps(ct: *mut u8, ss: *mut u8, pk: *const u8) -> i32 {
        unsafe { PQCLEAN_MLKEM512_CLEAN_crypto_kem_enc(ct, ss, pk) }
    }
    pub unsafe fn decaps(ss: *mut u8, ct: *const u8, sk: *const u8) -> i32 {
        unsafe { PQCLEAN_MLKEM512_CLEAN_crypto_kem_dec(ss, ct, sk) }
    }
}

#[cfg(feature = "mlkem1024")]
mod ffi {
    extern "C" {
        pub fn PQCLEAN_MLKEM1024_CLEAN_crypto_kem_keypair(pk: *mut u8, sk: *mut u8) -> i32;
        pub fn PQCLEAN_MLKEM1024_CLEAN_crypto_kem_enc(
            ct: *mut u8,
            ss: *mut u8,
            pk: *const u8,
        ) -> i32;
        pub fn PQCLEAN_MLKEM1024_CLEAN_crypto_kem_dec(
            ss: *mut u8,
            ct: *const u8,
            sk: *const u8,
        ) -> i32;
    }
    pub unsafe fn keypair(pk: *mut u8, sk: *mut u8) -> i32 {
        unsafe { PQCLEAN_MLKEM1024_CLEAN_crypto_kem_keypair(pk, sk) }
    }
    pub unsafe fn encaps(ct: *mut u8, ss: *mut u8, pk: *const u8) -> i32 {
        unsafe { PQCLEAN_MLKEM1024_CLEAN_crypto_kem_enc(ct, ss, pk) }
    }
    pub unsafe fn decaps(ss: *mut u8, ct: *const u8, sk: *const u8) -> i32 {
        unsafe { PQCLEAN_MLKEM1024_CLEAN_crypto_kem_dec(ss, ct, sk) }
    }
}

#[cfg(all(not(feature = "mlkem512"), not(feature = "mlkem768"), not(feature = "mlkem1024")))]
mod ffi {
    extern "C" {
        pub fn PQCLEAN_MLKEM768_CLEAN_crypto_kem_keypair(pk: *mut u8, sk: *mut u8) -> i32;
        pub fn PQCLEAN_MLKEM768_CLEAN_crypto_kem_enc(
            ct: *mut u8,
            ss: *mut u8,
            pk: *const u8,
        ) -> i32;
        pub fn PQCLEAN_MLKEM768_CLEAN_crypto_kem_dec(
            ss: *mut u8,
            ct: *const u8,
            sk: *const u8,
        ) -> i32;
    }
    pub unsafe fn keypair(pk: *mut u8, sk: *mut u8) -> i32 {
        unsafe { PQCLEAN_MLKEM768_CLEAN_crypto_kem_keypair(pk, sk) }
    }
    pub unsafe fn encaps(ct: *mut u8, ss: *mut u8, pk: *const u8) -> i32 {
        unsafe { PQCLEAN_MLKEM768_CLEAN_crypto_kem_enc(ct, ss, pk) }
    }
    pub unsafe fn decaps(ss: *mut u8, ct: *const u8, sk: *const u8) -> i32 {
        unsafe { PQCLEAN_MLKEM768_CLEAN_crypto_kem_dec(ss, ct, sk) }
    }
}

#[derive(Debug)]
pub enum KyberError {
    FfiError,
    InvalidLength,
}

#[inline]
fn ok(rc: i32) -> Result<(), KyberError> {
    if rc == 0 {
        Ok(())
    } else {
        Err(KyberError::FfiError)
    }
}

pub fn kyber_keygen() -> Result<KyberKeyPair, KyberError> {
    let mut pk = KyberPublicKey { bytes: [0u8; PUBLICKEY_BYTES] };
    let mut sk = KyberSecretKey { bytes: [0u8; SECRETKEY_BYTES] };
    let rc = unsafe { ffi::keypair(pk.bytes.as_mut_ptr(), sk.bytes.as_mut_ptr()) };
    ok(rc)?;
    Ok(KyberKeyPair { public_key: pk, secret_key: sk })
}

pub fn kyber_encaps(
    pk: &KyberPublicKey,
) -> Result<(KyberCiphertext, [u8; SHAREDSECRET_BYTES]), KyberError> {
    let mut ct = KyberCiphertext { bytes: [0u8; CIPHERTEXT_BYTES] };
    let mut ss = [0u8; SHAREDSECRET_BYTES];
    let rc = unsafe { ffi::encaps(ct.bytes.as_mut_ptr(), ss.as_mut_ptr(), pk.bytes.as_ptr()) };
    ok(rc)?;
    Ok((ct, ss))
}

pub fn kyber_decaps(
    ct: &KyberCiphertext,
    sk: &KyberSecretKey,
) -> Result<[u8; SHAREDSECRET_BYTES], KyberError> {
    let mut ss = [0u8; SHAREDSECRET_BYTES];
    let rc = unsafe { ffi::decaps(ss.as_mut_ptr(), ct.bytes.as_ptr(), sk.bytes.as_ptr()) };
    ok(rc)?;
    Ok(ss)
}

pub fn kyber_serialize_public_key(pk: &KyberPublicKey) -> Vec<u8> {
    pk.bytes.to_vec()
}
pub fn kyber_deserialize_public_key(data: &[u8]) -> Result<KyberPublicKey, KyberError> {
    if data.len() != PUBLICKEY_BYTES {
        return Err(KyberError::InvalidLength);
    }
    let mut bytes = [0u8; PUBLICKEY_BYTES];
    bytes.copy_from_slice(data);
    Ok(KyberPublicKey { bytes })
}
pub fn kyber_serialize_secret_key(sk: &KyberSecretKey) -> Vec<u8> {
    sk.bytes.to_vec()
}
pub fn kyber_deserialize_secret_key(data: &[u8]) -> Result<KyberSecretKey, KyberError> {
    if data.len() != SECRETKEY_BYTES {
        return Err(KyberError::InvalidLength);
    }
    let mut bytes = [0u8; SECRETKEY_BYTES];
    bytes.copy_from_slice(data);
    Ok(KyberSecretKey { bytes })
}
pub fn kyber_serialize_ciphertext(ct: &KyberCiphertext) -> Vec<u8> {
    ct.bytes.to_vec()
}
pub fn kyber_deserialize_ciphertext(data: &[u8]) -> Result<KyberCiphertext, KyberError> {
    if data.len() != CIPHERTEXT_BYTES {
        return Err(KyberError::InvalidLength);
    }
    let mut bytes = [0u8; CIPHERTEXT_BYTES];
    bytes.copy_from_slice(data);
    Ok(KyberCiphertext { bytes })
}
