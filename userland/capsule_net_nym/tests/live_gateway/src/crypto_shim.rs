//! The capsule's own crypto, pulled in by path so this test exercises the
//! shipping code rather than a copy of it. Only the pieces that are syscalls
//! in the capsule are stood in for here.

#[path = "../../../src/crypto/aes/mod.rs"]
pub mod aes;
#[path = "../../../src/crypto/polyval/mod.rs"]
pub mod polyval;
#[path = "../../../src/crypto/gcm_siv/mod.rs"]
pub mod gcm_siv;
#[path = "../../../src/crypto/hkdf_blake3/mod.rs"]
pub mod hkdf_blake3;
#[path = "../../../src/crypto/chacha20/mod.rs"]
pub mod chacha20;
#[path = "../../../src/crypto/blake2b/mod.rs"]
pub mod blake2b;
#[path = "../../../src/crypto/lioness/mod.rs"]
pub mod lioness;

/// HKDF and HMAC over SHA-256, which are syscalls in the capsule. Sphinx key
/// expansion uses these; the gateway handshake uses the BLAKE3 variant above.
pub mod kdf {
    use super::types::CryptoError;
    use hkdf::Hkdf;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    pub fn hkdf_sha256(salt: &[u8], ikm: &[u8], info: &[u8], out: &mut [u8]) -> Result<(), CryptoError> {
        Hkdf::<Sha256>::new(Some(salt), ikm).expand(info, out).map_err(|_| CryptoError::Kdf)
    }
    pub fn hmac_sha256(key: &[u8], data: &[u8], out: &mut [u8; 32]) -> Result<(), CryptoError> {
        let mut m = <Hmac<Sha256> as Mac>::new_from_slice(key).map_err(|_| CryptoError::Mac)?;
        m.update(data);
        out.copy_from_slice(&m.finalize().into_bytes());
        Ok(())
    }
}

pub mod types {
    #[derive(Debug, PartialEq)]
    pub enum CryptoError {
        Kdf,
        Mac,
        Ecdh,
    }
}

pub mod hash {
    use super::types::CryptoError;
    pub fn blake3(input: &[u8], out: &mut [u8; 32]) -> Result<(), CryptoError> {
        out.copy_from_slice(::blake3::hash(input).as_bytes());
        Ok(())
    }
}

pub mod ecdh {
    use super::types::CryptoError;
    use x25519_dalek::{PublicKey, StaticSecret};
    pub fn x25519_public(sk: &[u8; 32], out: &mut [u8; 32]) -> Result<(), CryptoError> {
        out.copy_from_slice(PublicKey::from(&StaticSecret::from(*sk)).as_bytes());
        Ok(())
    }
    pub fn x25519_shared(sk: &[u8; 32], pk: &[u8; 32], out: &mut [u8; 32]) -> Result<(), CryptoError> {
        out.copy_from_slice(
            StaticSecret::from(*sk).diffie_hellman(&PublicKey::from(*pk)).as_bytes(),
        );
        Ok(())
    }
}

pub mod random {
    use super::types::CryptoError;
    use rand::RngCore;
    pub fn fill_random(buf: &mut [u8]) -> Result<(), CryptoError> {
        rand::thread_rng().fill_bytes(buf);
        Ok(())
    }
}

// The capsule re-exports these at the crypto root, so the shim does too and
// code pulled in by path resolves the same paths either side.
pub use ecdh::{x25519_public, x25519_shared};
pub use random::fill_random;
