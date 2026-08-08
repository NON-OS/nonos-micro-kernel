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

use crate::syscall::abi::{tag4, AbiDomain, AbiEntry, AbiStatus};
use crate::syscall::numbers::SyscallNumber;

// Crypto family. CRND routes to the entropy capsule. CHSH, CENC,
// CDEC, and CEDV route to the crypto capsule.
pub(super) const ENTRIES: &[AbiEntry] = &[
    r(b"CRND", SyscallNumber::CryptoRandom, "CryptoRandom"),
    r(b"CHSH", SyscallNumber::CryptoHash, "CryptoHash"),
    r(b"CENC", SyscallNumber::CryptoEncrypt, "CryptoEncrypt"),
    r(b"CDEC", SyscallNumber::CryptoDecrypt, "CryptoDecrypt"),
    r(b"CEAD", SyscallNumber::CryptoEncryptAad, "CryptoEncryptAad"),
    r(b"CDAD", SyscallNumber::CryptoDecryptAad, "CryptoDecryptAad"),
    r(b"CEDV", SyscallNumber::CryptoEd25519Verify, "CryptoEd25519Verify"),
    r(b"CEDS", SyscallNumber::CryptoEd25519Sign, "CryptoEd25519Sign"),
    r(b"CEDP", SyscallNumber::CryptoEd25519Pubkey, "CryptoEd25519Pubkey"),
    r(b"CXPK", SyscallNumber::CryptoX25519Public, "CryptoX25519Public"),
    r(b"CXSH", SyscallNumber::CryptoX25519Shared, "CryptoX25519Shared"),
    r(b"CHMC", SyscallNumber::CryptoHmacSha256, "CryptoHmacSha256"),
    r(b"CHKF", SyscallNumber::CryptoHkdfSha256, "CryptoHkdfSha256"),
    r(b"CKEC", SyscallNumber::CryptoKeccak256, "CryptoKeccak256"),
    r(b"CSKS", SyscallNumber::CryptoSecp256k1Sign, "CryptoSecp256k1Sign"),
    r(b"CSPB", SyscallNumber::CryptoSecp256k1Pubkey, "CryptoSecp256k1Pubkey"),
];

const fn r(tag: &[u8; 4], variant: SyscallNumber, name: &'static str) -> AbiEntry {
    AbiEntry { id: tag4(tag), variant, name, domain: AbiDomain::Crypto, status: AbiStatus::Routed }
}
