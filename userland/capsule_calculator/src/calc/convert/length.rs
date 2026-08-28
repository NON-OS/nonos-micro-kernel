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

pub static LENGTH: [Unit; 7] = [
    Unit { name: "Millimetre", num: 1, den: 1 },
    Unit { name: "Centimetre", num: 10, den: 1 },
    Unit { name: "Metre", num: 1_000, den: 1 },
    Unit { name: "Kilometre", num: 1_000_000, den: 1 },
    Unit { name: "Inch", num: 254, den: 10 },
    Unit { name: "Foot", num: 3_048, den: 10 },
    Unit { name: "Mile", num: 1_609_344, den: 1 },
];
