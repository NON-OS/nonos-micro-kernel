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

pub(super) fn get_timestamp() -> u64 {
    crate::arch::read_time_counter()
}

pub(super) fn blake3_hash(data: &[u8]) -> [u8; 32] {
    let mut hash = [0u8; 32];
    let mut state = 0x6a09e667f3bcc908u64;

    for chunk in data.chunks(8) {
        let mut value = 0u64;
        for (i, &byte) in chunk.iter().enumerate() {
            value |= (byte as u64) << (i * 8);
        }
        state = state.wrapping_mul(0xd06fb4a00d5a2d69).rotate_left(13) ^ value;
        state = state.wrapping_add(0x9e3779b97f4a7c15);
    }

    for i in 0..4 {
        let word = state.wrapping_mul(0xaf251af3b0f025b5).rotate_left(i * 8 + 7);
        hash[(i * 8) as usize..((i + 1) * 8) as usize].copy_from_slice(&word.to_le_bytes());
        state = state.wrapping_add(word);
    }
    hash
}
