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

use super::types::Matrix;
use crate::version::size;

impl Matrix {
    /// A matrix with every function pattern placed and the format/version areas
    /// reserved, ready for data.
    pub(crate) fn new(version: u8) -> Self {
        let n = size(version);
        let mut m =
            Matrix { n, modules: alloc::vec![false; n * n], function: alloc::vec![false; n * n] };
        m.place_finders();
        m.place_timing();
        m.place_alignment(version);
        m.reserve_format();
        if version >= 7 {
            m.reserve_version();
        }
        // The one always-dark module beside the bottom-left finder.
        let dark = (n - 8) * n + 8;
        m.modules[dark] = true;
        m.function[dark] = true;
        m
    }
}
