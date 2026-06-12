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

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub use super::super::pqc::kyber;
pub use super::super::pqc::mceliece;
#[cfg(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5"))]
pub use super::super::pqc::ml_dsa_65;
pub use super::super::pqc::ntru;
pub use super::super::pqc::quantum;
pub use super::super::pqc::sphincs;

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub use super::super::pqc::kyber::{
    kyber_decaps, kyber_deserialize_ciphertext, kyber_deserialize_public_key,
    kyber_deserialize_secret_key, kyber_encaps, kyber_keygen, kyber_serialize_ciphertext,
    kyber_serialize_public_key, kyber_serialize_secret_key, KyberCiphertext, KyberKeyPair,
    KyberPublicKey, KyberSecretKey, CIPHERTEXT_BYTES as KYBER_CT_BYTES, KYBER_PARAM_NAME,
    PUBLICKEY_BYTES as KYBER_PUB_BYTES, SECRETKEY_BYTES as KYBER_SK_BYTES,
};

#[cfg(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5"))]
pub use super::super::pqc::ml_dsa_65::{
    ml_dsa_65_deserialize_public_key, ml_dsa_65_deserialize_secret_key,
    ml_dsa_65_deserialize_signature, ml_dsa_65_keypair, ml_dsa_65_serialize_public_key,
    ml_dsa_65_serialize_secret_key, ml_dsa_65_serialize_signature, ml_dsa_65_sign,
    ml_dsa_65_verify, MlDsa65KeyPair, MlDsa65PublicKey, MlDsa65SecretKey, MlDsa65Signature,
    PARAM_NAME, PUBLICKEY_BYTES as MLDSA65_PUB_BYTES, SECRETKEY_BYTES as MLDSA65_SK_BYTES,
    SIGNATURE_BYTES as MLDSA65_SIG_BYTES,
};
