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

extern crate alloc;

mod commit;
mod prove_plonk;
mod prove_range;
mod verify;

pub use commit::syscall_zk_commit;
pub use prove_plonk::syscall_zk_prove_plonk;
pub use prove_range::syscall_zk_prove_range;
pub use verify::syscall_zk_verify;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZkError {
    Success = 0,
    InvalidProof = 1,
    MalformedInput = 2,
    UnsupportedProofType = 3,
    InternalError = 4,
    PermissionDenied = 5,
}
