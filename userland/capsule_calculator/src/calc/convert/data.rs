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

pub static DATA: [Unit; 6] = [
    Unit { name: "Bit", num: 1, den: 8 },
    Unit { name: "Byte", num: 1, den: 1 },
    Unit { name: "Kibibyte", num: 1_024, den: 1 },
    Unit { name: "Mebibyte", num: 1_048_576, den: 1 },
    Unit { name: "Gibibyte", num: 1_073_741_824, den: 1 },
    Unit { name: "Tebibyte", num: 1_099_511_627_776, den: 1 },
];
