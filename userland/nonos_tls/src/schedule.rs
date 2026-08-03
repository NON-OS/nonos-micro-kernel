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

use super::traffic_keys::TrafficKeys;

pub(super) const EMPTY_HASH: [u8; 32] = [
    0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
    0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
];

pub fn handshake_keys(shared: &[u8; 32], transcript: &[u8], suite: u16) -> Option<TrafficKeys> {
    let zero = [0u8; 32];
    let early = super::hkdf::extract(&zero, &zero)?;
    let derived = secret(&early, b"derived", &EMPTY_HASH)?;
    let handshake = super::hkdf::extract(&derived, shared)?;
    let th = super::hash_sha256::hash_sha256(transcript)?;
    let client_secret = secret(&handshake, b"c hs traffic", &th)?;
    let server_secret = secret(&handshake, b"s hs traffic", &th)?;
    Some(TrafficKeys {
        suite,
        handshake_secret: handshake,
        client_secret,
        server_secret,
        client_key: key(&client_secret, suite)?,
        client_iv: iv(&client_secret)?,
        server_key: key(&server_secret, suite)?,
        server_iv: iv(&server_secret)?,
    })
}

pub(super) fn secret(base: &[u8; 32], label: &[u8], context: &[u8; 32]) -> Option<[u8; 32]> {
    let mut out = [0u8; 32];
    if super::expand_label::expand_label(base, label, context, &mut out) {
        Some(out)
    } else {
        None
    }
}

// Derive the record key. AES-128-GCM uses a 16-byte key, ChaCha20-Poly1305 a
// 32-byte key; the HKDF-Expand-Label length must match, so the shorter key is
// derived into the low bytes and the record layer reads only what its cipher
// needs.
pub(super) fn key(secret: &[u8; 32], suite: u16) -> Option<[u8; 32]> {
    let mut out = [0u8; 32];
    let len = if suite == super::constants::SUITE_AES128_GCM_SHA256 { 16 } else { 32 };
    if super::expand_label::expand_label(secret, b"key", &[], &mut out[..len]) {
        Some(out)
    } else {
        None
    }
}

pub(super) fn iv(secret: &[u8; 32]) -> Option<[u8; 12]> {
    let mut out = [0u8; 12];
    if super::expand_label::expand_label(secret, b"iv", &[], &mut out) {
        Some(out)
    } else {
        None
    }
}
