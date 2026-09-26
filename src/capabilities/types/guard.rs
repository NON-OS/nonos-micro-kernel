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

//! What the capability list has to satisfy, checked when the kernel is built
//! rather than trusted.

use super::Capability;

const _: () = {
    let caps = Capability::all();
    let mut seen = 0u64;
    let mut i = 0;
    while i < caps.len() {
        let bit = caps[i].bit();
        assert!(bit.count_ones() == 1, "a capability must occupy exactly one bit");
        assert!(seen & bit == 0, "two capabilities share a bit");
        seen |= bit;
        i += 1;
    }
};
