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

// Ed25519's `KeyPair::generate` is the only consumer of the RNG, and the KATs
// exercise sign/verify with fixed seeds from the standard vectors. This shim
// satisfies the include without pulling the kernel's entropy stack; nothing in
// the proof set calls it.
pub fn get_random_bytes() -> [u8; 32] {
    [0u8; 32]
}
