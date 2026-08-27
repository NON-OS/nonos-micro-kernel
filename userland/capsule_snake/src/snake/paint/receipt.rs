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

use crate::snake::state::Game;

const OFFSET: u32 = 2_166_136_261;
const PRIME: u32 = 16_777_619;

fn mix(hash: u32, value: u32) -> u32 {
    let mut hash = hash;
    for shift in 0..4 {
        hash ^= (value >> (shift * 8)) & 0xFF;
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

// A run is identified by what it actually was, not by a counter, so the same
// score over a different board and clock reads as a different receipt.
pub fn of(game: &Game) -> u32 {
    let mut hash = mix(OFFSET, game.score);
    hash = mix(hash, game.body.len() as u32);
    hash = mix(hash, game.level as u32);
    hash = mix(hash, game.streak);
    hash = mix(hash, game.longest as u32);
    mix(hash, (game.elapsed / 100) as u32)
}
