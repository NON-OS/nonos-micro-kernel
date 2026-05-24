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

pub const HYBRID_SCHEME: &[u8] = b"Ed25519 + ML-DSA-65 (hybrid)";
pub const MANIFEST_FORMAT: &[u8] = b"capsule_manifest v3";
pub const CERT_FORMAT: &[u8] = b"NONOS-ID cert hybrid";
pub const SIGNING_CHAIN: &[u8] = b"trust-anchor -> publisher -> capsule";
pub const STATUS: &[u8] =
    b"reached _start, which means capsule_spawn::spawn_verified accepted the cert + manifest";
