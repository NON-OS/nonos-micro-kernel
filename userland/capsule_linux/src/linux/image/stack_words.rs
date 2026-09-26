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

//! The word list a program finds at rsp.

use alloc::vec::Vec;

/// `at` is every string address, argv first then envp, and `argc` says where
/// the boundary falls.
pub fn words(argc: usize, at: &[u64], aux: Vec<u64>) -> Option<Vec<u64>> {
    let mut out: Vec<u64> = alloc::vec![argc as u64];
    out.extend_from_slice(at.get(..argc)?);
    out.push(0);
    out.extend_from_slice(at.get(argc..)?);
    out.push(0);
    out.extend(aux);
    Some(out)
}
