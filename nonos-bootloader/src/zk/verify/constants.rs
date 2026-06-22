// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

/// ** DS = Domain separator **
pub const DS_PROGRAM_HASH: &str = "NONOS:ZK:PROGRAM:v1";
/// ** Maximum proof size (2 MB) **
pub const MAX_PROOF_SIZE: usize = 2 * 1024 * 1024;
/// ** Maximum public inputs size (256 KB) **
pub const MAX_INPUT_SIZE: usize = 256 * 1024;
const _: () = assert!(MAX_INPUT_SIZE % 32 == 0);
