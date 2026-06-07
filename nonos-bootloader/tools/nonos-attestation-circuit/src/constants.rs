// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

pub const DS_PROGRAM: &str = "NONOS:ZK:PROGRAM:v1";

pub const DS_COMMITMENT: &str = "NONOS:CAPSULE:COMMITMENT:v1";

pub const DS_SOURCE_TREE: &str = "NONOS:BUILD:SOURCE_TREE:v1";

pub const DS_CARGO_LOCK: &str = "NONOS:BUILD:CARGO_LOCK:v1";

pub const DS_RUSTC_VERSION: &str = "NONOS:BUILD:RUSTC_VERSION:v1";

pub const DS_BUILD_CONFIG: &str = "NONOS:BUILD:CONFIG:v1";

pub const DS_BUILD_PROVENANCE: &str = "NONOS:BUILD:PROVENANCE:v1";

pub const MIN_HW_LEVEL: u64 = 0x1000;

pub const PCR_PREIMAGE_LEN: usize = 64;

pub const MIN_PCR_ENTROPY_BYTES: usize = 32;

pub const GROTH16_PROOF_SIZE: usize = 192;

pub const BUILD_PROVENANCE_HASH_COUNT: usize = 4;

#[derive(Clone, Copy, Debug)]
pub struct BuildProvenance {
    pub source_tree_hash: [u8; 32],
    pub cargo_lock_hash: [u8; 32],
    pub rustc_version_hash: [u8; 32],
    pub build_config_hash: [u8; 32],
}

impl BuildProvenance {
    pub fn new(
        source_tree_hash: [u8; 32],
        cargo_lock_hash: [u8; 32],
        rustc_version_hash: [u8; 32],
        build_config_hash: [u8; 32],
    ) -> Self {
        Self { source_tree_hash, cargo_lock_hash, rustc_version_hash, build_config_hash }
    }

    pub fn compute_composite_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new_derive_key(DS_BUILD_PROVENANCE);
        hasher.update(&self.source_tree_hash);
        hasher.update(&self.cargo_lock_hash);
        hasher.update(&self.rustc_version_hash);
        hasher.update(&self.build_config_hash);
        *hasher.finalize().as_bytes()
    }

    pub fn to_bytes(&self) -> [u8; 128] {
        let mut out = [0u8; 128];
        out[0..32].copy_from_slice(&self.source_tree_hash);
        out[32..64].copy_from_slice(&self.cargo_lock_hash);
        out[64..96].copy_from_slice(&self.rustc_version_hash);
        out[96..128].copy_from_slice(&self.build_config_hash);
        out
    }

    pub fn from_bytes(bytes: &[u8; 128]) -> Self {
        let mut source_tree_hash = [0u8; 32];
        let mut cargo_lock_hash = [0u8; 32];
        let mut rustc_version_hash = [0u8; 32];
        let mut build_config_hash = [0u8; 32];
        source_tree_hash.copy_from_slice(&bytes[0..32]);
        cargo_lock_hash.copy_from_slice(&bytes[32..64]);
        rustc_version_hash.copy_from_slice(&bytes[64..96]);
        build_config_hash.copy_from_slice(&bytes[96..128]);
        Self { source_tree_hash, cargo_lock_hash, rustc_version_hash, build_config_hash }
    }
}

#[inline]
pub fn expected_program_hash_bytes() -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key(DS_PROGRAM);
    h.update(b"zkmod-attestation-program-v1");
    *h.finalize().as_bytes()
}

#[inline]
pub fn compute_capsule_commitment(public_inputs: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key(DS_COMMITMENT);
    hasher.update(public_inputs);
    *hasher.finalize().as_bytes()
}

#[inline]
pub fn compute_source_tree_hash(git_commit: &[u8], tree_hash: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key(DS_SOURCE_TREE);
    hasher.update(git_commit);
    hasher.update(tree_hash);
    *hasher.finalize().as_bytes()
}

#[inline]
pub fn compute_cargo_lock_hash(cargo_lock_contents: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key(DS_CARGO_LOCK);
    hasher.update(cargo_lock_contents);
    *hasher.finalize().as_bytes()
}

#[inline]
pub fn compute_rustc_version_hash(version_string: &[u8], commit_hash: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key(DS_RUSTC_VERSION);
    hasher.update(version_string);
    hasher.update(commit_hash);
    *hasher.finalize().as_bytes()
}

#[inline]
pub fn compute_build_config_hash(cargo_toml: &[u8], cargo_config: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key(DS_BUILD_CONFIG);
    hasher.update(cargo_toml);
    hasher.update(cargo_config);
    *hasher.finalize().as_bytes()
}
