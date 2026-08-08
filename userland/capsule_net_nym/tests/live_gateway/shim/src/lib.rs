//! Host stand-in for the two syscalls the handshake reaches for. Same
//! signatures and same return conventions as the capsule's libc, so the
//! handshake code under test is unmodified.
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

/// Returns 64 on success, as the kernel wrapper does.
pub extern "C" fn crypto_ed25519_sign(
    seed: *const u8, message: *const u8, message_len: usize, out: *mut u8,
) -> i64 {
    unsafe {
        let mut s = [0u8; 32];
        s.copy_from_slice(core::slice::from_raw_parts(seed, 32));
        let msg = core::slice::from_raw_parts(message, message_len);
        let sig = SigningKey::from_bytes(&s).sign(msg);
        core::ptr::copy_nonoverlapping(sig.to_bytes().as_ptr(), out, 64);
    }
    64
}

/// Returns 0 when the signature checks, as the kernel wrapper does.
pub extern "C" fn crypto_ed25519_verify(
    pubkey: *const u8, signature: *const u8, message: *const u8, message_len: usize,
) -> i64 {
    unsafe {
        let mut pk = [0u8; 32];
        pk.copy_from_slice(core::slice::from_raw_parts(pubkey, 32));
        let mut sg = [0u8; 64];
        sg.copy_from_slice(core::slice::from_raw_parts(signature, 64));
        let msg = core::slice::from_raw_parts(message, message_len);
        match VerifyingKey::from_bytes(&pk) {
            Ok(vk) => match vk.verify(msg, &Signature::from_bytes(&sg)) {
                Ok(()) => 0,
                Err(_) => -74,
            },
            Err(_) => -22,
        }
    }
}
