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

use super::units::Unit;

pub static WEIGHT: [Unit; 6] = [
    Unit { name: "Milligram", num: 1, den: 1_000 },
    Unit { name: "Gram", num: 1, den: 1 },
    Unit { name: "Kilogram", num: 1_000, den: 1 },
    Unit { name: "Tonne", num: 1_000_000, den: 1 },
    Unit { name: "Ounce", num: 28_349_523, den: 1_000_000 },
    Unit { name: "Pound", num: 45_359_237, den: 100_000 },
];
