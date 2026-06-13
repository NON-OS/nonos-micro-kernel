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

use core::hash::{BuildHasher, Hasher};

use super::seed::process_seed;

const PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Copy)]
pub struct SeededState {
    key: u64,
}

impl Default for SeededState {
    fn default() -> Self {
        Self { key: process_seed() }
    }
}

impl BuildHasher for SeededState {
    type Hasher = SeededHasher;
    fn build_hasher(&self) -> SeededHasher {
        SeededHasher { hash: self.key }
    }
}

pub struct SeededHasher {
    hash: u64,
}

impl Hasher for SeededHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.hash = (self.hash ^ b as u64).wrapping_mul(PRIME);
        }
    }

    fn finish(&self) -> u64 {
        let mut h = self.hash;
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= h >> 33;
        h
    }
}
