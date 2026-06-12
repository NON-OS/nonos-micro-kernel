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

use super::empty_rail::empty_rail;
use super::rail_allowed::rail_allowed;
use super::types::{Rail, MAX_RAILS};

pub fn filter_rails(rails: &mut [Rail; MAX_RAILS], count: usize) -> usize {
    let mut n = 0;
    let limit = if count > MAX_RAILS { MAX_RAILS } else { count };
    for i in 0..limit {
        if rail_allowed(&rails[i]) {
            rails[n] = rails[i];
            n += 1;
        }
    }
    for rail in rails.iter_mut().take(MAX_RAILS).skip(n) {
        *rail = empty_rail();
    }
    n
}
