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

pub const MAGIC: u32 = 0x4E4F_4358; // "NOCX" — NONOS Crypto
pub const VERSION: u16 = 1;

pub const OP_BLAKE3_HASH: u16 = 1;
pub const OP_SHA3_256_HASH: u16 = 2;
pub const OP_HEALTHCHECK: u16 = 3;
pub const OP_SHA256_HASH: u16 = 4;
pub const OP_SHA512_HASH: u16 = 5;
pub const OP_ED25519_VERIFY: u16 = 6;
pub const OP_CHACHA20_POLY1305_SEAL: u16 = 10;
pub const OP_CHACHA20_POLY1305_OPEN: u16 = 11;
pub const OP_AES256_GCM_SEAL: u16 = 12;
pub const OP_AES256_GCM_OPEN: u16 = 13;

// Ed25519 verify input layout: pubkey[32] || sig[64] || message.
// Message length is bounded so a hostile caller cannot park a
// multi-megabyte buffer in the capsule heap; the canonical
// marketplace index fits well within this cap.
pub const ED25519_PUBKEY_BYTES: u32 = 32;
pub const ED25519_SIG_BYTES: u32 = 64;
pub const ED25519_HEADER_BYTES: u32 = ED25519_PUBKEY_BYTES + ED25519_SIG_BYTES;
pub const MAX_VERIFY_MESSAGE_BYTES: u32 = 1024 * 1024;

pub const MAX_INPUT_BYTES: u32 = 65536;
pub const MAX_AEAD_PT_BYTES: u32 = 1024 * 1024;
pub const MAX_AEAD_AAD_BYTES: u32 = 256;
pub const AEAD_KEY_BYTES: u32 = 32;
pub const AEAD_NONCE_BYTES: u32 = 12;
pub const AEAD_TAG_BYTES: u32 = 16;
pub const AEAD_HEADER_BYTES: u32 = AEAD_KEY_BYTES + AEAD_NONCE_BYTES + 4;
// Per-op payload caps are enforced by each handler. The shared
// envelope budget is the larger of the AEAD plaintext path and
// the Ed25519 verify path, with a small cushion for headers.
pub const MAX_PAYLOAD_BYTES: u32 = {
    let aead = AEAD_HEADER_BYTES + MAX_AEAD_AAD_BYTES + MAX_AEAD_PT_BYTES + AEAD_TAG_BYTES;
    let verify = ED25519_HEADER_BYTES + MAX_VERIFY_MESSAGE_BYTES;
    if aead > verify {
        aead
    } else {
        verify
    }
};

// Distinct from ramfs (4294967297), keyring (4294967298), entropy
// (4294967299) so concurrent in-flight requests cannot cross-route.
pub const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_0004;

// Same 20-byte header shape as capsule_entropy: u32 magic, u16 version,
// u16 op, u16 flags, u16 _reserved, u32 request_id, u32 payload_len.
pub const HDR_LEN: usize = 20;

#[derive(Clone, Copy)]
pub struct Request<'a> {
    pub op: u16,
    pub flags: u16,
    pub request_id: u32,
    pub payload: &'a [u8],
}
