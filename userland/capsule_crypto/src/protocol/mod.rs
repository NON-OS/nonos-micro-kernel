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

mod decode;
mod encode;
mod errno;
mod primitives;
mod types;

pub use decode::decode_request;
pub use encode::encode_response;
pub use errno::{EBADMSG, EINVAL, EIO, EMSGSIZE};
pub use types::{
    Request, AEAD_KEY_BYTES, AEAD_NONCE_BYTES, AEAD_TAG_BYTES,
    ED25519_HEADER_BYTES, ED25519_PUBKEY_BYTES, ED25519_SIG_BYTES, HDR_LEN, KERNEL_REPLY_ENDPOINT,
    MAX_AEAD_AAD_BYTES, MAX_AEAD_PT_BYTES, MAX_INPUT_BYTES, MAX_PAYLOAD_BYTES,
    MAX_VERIFY_MESSAGE_BYTES, OP_AES256_GCM_OPEN, OP_AES256_GCM_SEAL,
    OP_BLAKE3_HASH, OP_CHACHA20_POLY1305_OPEN, OP_CHACHA20_POLY1305_SEAL, OP_ED25519_VERIFY,
    OP_HEALTHCHECK, OP_SHA256_HASH, OP_SHA3_256_HASH, OP_SHA512_HASH,
};
pub use primitives::{
    HMAC_KEY_MAX, HKDF_OUT_MAX, HKDF_PART_MAX, OP_HKDF_SHA256, OP_HMAC_SHA256, OP_X25519_PUBLIC,
    OP_P256_ECDSA_VERIFY, OP_P384_ECDSA_VERIFY, OP_SHA384_HASH, OP_X25519_SHARED,
    P256_VERIFY_BYTES, P384_VERIFY_BYTES, X25519_KEY_BYTES,
};
