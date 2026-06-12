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

use alloc::vec::Vec;

// The four bytes blobs that make up a capsule, copied out of user memory by the
// load syscall. They are owned here and promoted to a static lifetime before
// the verified spawn, exactly as the embedded artifacts are.
pub struct CapsuleArtifacts {
    pub elf: Vec<u8>,
    pub cert: Vec<u8>,
    pub manifest: Vec<u8>,
    pub trailer: Vec<u8>,
}
