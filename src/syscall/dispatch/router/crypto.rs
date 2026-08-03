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

use crate::syscall::dispatch::crypto::{
    handle_crypto_decrypt, handle_crypto_decrypt_aad, handle_crypto_ed25519_pubkey,
    handle_crypto_ed25519_sign,
    handle_crypto_ed25519_verify, handle_crypto_encrypt, handle_crypto_encrypt_aad,
    handle_crypto_hash, handle_crypto_keccak256, handle_crypto_random,
    handle_crypto_secp256k1_pubkey, handle_crypto_secp256k1_sign, handle_hkdf_sha256,
    handle_hmac_sha256, handle_x25519_public, handle_x25519_shared,
};
use crate::syscall::dispatch::util::errno;
use crate::syscall::numbers::SyscallNumber;
use crate::syscall::SyscallResult;

pub(super) fn dispatch_crypto(
    syscall: SyscallNumber,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    _a5: u64,
) -> SyscallResult {
    match syscall {
        SyscallNumber::CryptoRandom => handle_crypto_random(a0, a1),
        SyscallNumber::CryptoHash => handle_crypto_hash(a0, a1, a2, a3, a4),
        SyscallNumber::CryptoEncrypt => handle_crypto_encrypt(a0, a1, a2, a3, a4, _a5),
        SyscallNumber::CryptoDecrypt => handle_crypto_decrypt(a0, a1, a2, a3, a4, _a5),
        SyscallNumber::CryptoEncryptAad => handle_crypto_encrypt_aad(a0, a1, a2, a3, a4, _a5),
        SyscallNumber::CryptoDecryptAad => handle_crypto_decrypt_aad(a0, a1, a2, a3, a4, _a5),
        SyscallNumber::CryptoEd25519Verify => handle_crypto_ed25519_verify(a0, a1, a2, a3),
        SyscallNumber::CryptoEd25519Sign => handle_crypto_ed25519_sign(a0, a1, a2, a3),
        SyscallNumber::CryptoEd25519Pubkey => handle_crypto_ed25519_pubkey(a0, a1),
        SyscallNumber::CryptoX25519Public => handle_x25519_public(a0, a1),
        SyscallNumber::CryptoX25519Shared => handle_x25519_shared(a0, a1, a2),
        SyscallNumber::CryptoHmacSha256 => handle_hmac_sha256(a0, a1, a2, a3, a4),
        SyscallNumber::CryptoHkdfSha256 => handle_hkdf_sha256(a0, a1, a2, a3),
        SyscallNumber::CryptoKeccak256 => handle_crypto_keccak256(a0, a1, a2, a3),
        SyscallNumber::CryptoSecp256k1Sign => handle_crypto_secp256k1_sign(a0, a1, a2),
        SyscallNumber::CryptoSecp256k1Pubkey => handle_crypto_secp256k1_pubkey(a0, a1),
        _ => errno(38),
    }
}
