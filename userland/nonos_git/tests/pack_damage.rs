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
//! Packs that were altered after they were sealed.

mod reseal;

use nonos_git::{read_pack, PackError};
use reseal::reseal;

const SIMPLE: &[u8] = include_bytes!("data/simple.pack");

#[test]
fn a_single_flipped_bit_is_caught() {
    let mut bad = SIMPLE.to_vec();
    bad[30] ^= 0x01;
    assert_eq!(read_pack(&bad).err(), Some(PackError::Checksum));
}

#[test]
fn an_object_swapped_for_another_is_caught() {
    // Resealing hides the tampering from the trailer, so what catches it is
    // the id being recomputed from the bytes rather than read from the pack.
    let mut bad = SIMPLE.to_vec();
    let at = bad.len() - 30;
    bad[at] ^= 0x40;
    reseal(&mut bad);
    assert!(read_pack(&bad).is_err());
}
