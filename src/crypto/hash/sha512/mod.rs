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

mod constants;
mod hasher;

pub use hasher::Sha512;

pub type Hash512 = [u8; 64];

pub fn sha512(data: &[u8]) -> Hash512 {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize()
}

#[inline]
pub fn sha512_hash(data: &[u8]) -> Hash512 {
    sha512(data)
}
