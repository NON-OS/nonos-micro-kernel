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
