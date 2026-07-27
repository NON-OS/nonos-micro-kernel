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

use super::version_bits::version_bits;
use crate::matrix::Matrix;

/// Write both copies of the version information (no-op below version 7).
pub(crate) fn write_version(m: &mut Matrix, version: u8) {
    if version < 7 {
        return;
    }
    let bits = version_bits(version);
    let n = m.n;
    for i in 0..18usize {
        let b = (bits >> i) & 1 == 1;
        let (r, c) = (i / 3, i % 3);
        m.set_format(r, n - 11 + c, b);
        m.set_format(n - 11 + c, r, b);
    }
}
