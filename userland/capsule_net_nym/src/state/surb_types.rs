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

pub(super) const CAP: usize = 64;
pub(super) const DEFAULT_TTL_MS: u64 = 600_000;
const MIN_TTL_MS: u64 = 30_000;
const MAX_TTL_MS: u64 = 86_400_000;

#[derive(Clone, Copy)]
pub(super) struct Surb {
    pub owner: u32,
    pub id: u32,
    pub session: u32,
    pub tag: [u8; 32],
    pub expires_ms: u64,
}

pub(super) fn normalize_ttl(ttl_ms: u64) -> u64 {
    if ttl_ms == 0 {
        DEFAULT_TTL_MS
    } else {
        ttl_ms.clamp(MIN_TTL_MS, MAX_TTL_MS)
    }
}
