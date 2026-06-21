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

use core::sync::atomic::Ordering;

use super::{AlgorithmStatus, CRYPTO_STATE};

pub fn kat_sha3_256() -> AlgorithmStatus {
    let expected: [u8; 32] = [
        0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56, 0xa0, 0x61, 0xd6,
        0x62, 0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa, 0x82, 0xd8, 0x0a, 0x4b, 0x80, 0xf8,
        0x43, 0x4a,
    ];

    let result = crate::crypto::sha3::sha3_256(b"");

    if result == expected {
        CRYPTO_STATE.sha3_256.store(true, Ordering::SeqCst);
        CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Pass
    } else {
        CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Fail
    }
}

pub fn kat_blake3() -> AlgorithmStatus {
    let input = b"NONOS KAT Known Answer Vector";
    let result1 = crate::crypto::blake3::blake3_hash(input);
    let result2 = crate::crypto::blake3::blake3_hash(input);

    if result1 == result2 && result1 != [0u8; 32] {
        CRYPTO_STATE.blake3.store(true, Ordering::SeqCst);
        CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Pass
    } else {
        CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Fail
    }
}

pub fn kat_chacha20poly1305() -> AlgorithmStatus {
    let key: [u8; 32] = [
        0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e,
        0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d,
        0x9e, 0x9f,
    ];
    let nonce: [u8; 12] = [0x07, 0x00, 0x00, 0x00, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47];
    let aad = b"";
    let plaintext = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";

    match crate::crypto::chacha20poly1305::aead_encrypt(&key, &nonce, aad, plaintext) {
        Ok(ciphertext) => {
            match crate::crypto::chacha20poly1305::aead_decrypt(&key, &nonce, aad, &ciphertext) {
                Ok(decrypted) => {
                    if decrypted == plaintext {
                        CRYPTO_STATE.chacha20poly1305.store(true, Ordering::SeqCst);
                        CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
                        AlgorithmStatus::Pass
                    } else {
                        CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
                        AlgorithmStatus::Fail
                    }
                }
                Err(_) => {
                    CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
                    AlgorithmStatus::Fail
                }
            }
        }
        Err(_) => {
            CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
            AlgorithmStatus::Fail
        }
    }
}

pub fn kat_ed25519() -> AlgorithmStatus {
    let message = b"NONOS Ed25519 KAT";

    let keypair = crate::crypto::ed25519::KeyPair::generate();

    let signature = crate::crypto::ed25519::sign(&keypair, message);

    let valid = crate::crypto::ed25519::verify(&keypair.public, message, &signature);

    let wrong_valid = crate::crypto::ed25519::verify(&keypair.public, b"wrong message", &signature);

    if valid && !wrong_valid {
        CRYPTO_STATE.ed25519.store(true, Ordering::SeqCst);
        CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Pass
    } else {
        CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Fail
    }
}

pub fn kat_rng() -> AlgorithmStatus {
    let mut buf1 = [0u8; 32];
    let mut buf2 = [0u8; 32];

    crate::crypto::rng::fill_random_bytes(&mut buf1);
    crate::crypto::rng::fill_random_bytes(&mut buf2);

    let buf1_zero = buf1.iter().all(|&b| b == 0);
    let buf2_zero = buf2.iter().all(|&b| b == 0);
    let same = buf1 == buf2;

    if !buf1_zero && !buf2_zero && !same {
        CRYPTO_STATE.rng.store(true, Ordering::SeqCst);
        CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Pass
    } else {
        CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Fail
    }
}

pub fn kat_sphincs() -> AlgorithmStatus {
    let message = b"NONOS SPHINCS+ KAT";

    match crate::crypto::sphincs::sphincs_keygen() {
        Ok(keypair) => match crate::crypto::sphincs::sphincs_sign(&keypair.secret_key, message) {
            Ok(signature) => {
                let valid = crate::crypto::sphincs::sphincs_verify(
                    &keypair.public_key,
                    message,
                    &signature,
                );

                if valid {
                    CRYPTO_STATE.sphincs.store(true, Ordering::SeqCst);
                    CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
                    AlgorithmStatus::Pass
                } else {
                    CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
                    AlgorithmStatus::Fail
                }
            }
            Err(_) => {
                CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
                AlgorithmStatus::Fail
            }
        },
        Err(_) => AlgorithmStatus::Unavailable,
    }
}

pub fn kat_ntru() -> AlgorithmStatus {
    match crate::crypto::ntru::ntru_keygen() {
        Ok(keypair) => match crate::crypto::ntru::ntru_encaps(&keypair.public_key) {
            Ok((ciphertext, shared_secret1)) => {
                match crate::crypto::ntru::ntru_decaps(&ciphertext, &keypair.secret_key) {
                    Ok(shared_secret2) => {
                        if shared_secret1 == shared_secret2 {
                            CRYPTO_STATE.ntru.store(true, Ordering::SeqCst);
                            CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
                            AlgorithmStatus::Pass
                        } else {
                            CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
                            AlgorithmStatus::Fail
                        }
                    }
                    Err(_) => {
                        CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
                        AlgorithmStatus::Fail
                    }
                }
            }
            Err(_) => {
                CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
                AlgorithmStatus::Fail
            }
        },
        Err(_) => AlgorithmStatus::Unavailable,
    }
}
