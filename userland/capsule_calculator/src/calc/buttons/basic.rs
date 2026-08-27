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

use super::kinds::Button;
use super::{row_four, row_function, row_memory, row_one, row_seven, row_zero};

pub static ROWS: [&[Button]; 6] = [
    &row_memory::ROW,
    &row_function::ROW,
    &row_seven::ROW,
    &row_four::ROW,
    &row_one::ROW,
    &row_zero::ROW,
];
