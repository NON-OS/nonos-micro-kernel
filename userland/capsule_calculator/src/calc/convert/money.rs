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

pub static CURRENCY: [Unit; 7] = [
    Unit { name: "US Dollar", num: 100, den: 100 },
    Unit { name: "Euro", num: 108, den: 100 },
    Unit { name: "Pound Sterling", num: 127, den: 100 },
    Unit { name: "Swiss Franc", num: 112, den: 100 },
    Unit { name: "Canadian Dollar", num: 74, den: 100 },
    Unit { name: "Australian Dollar", num: 66, den: 100 },
    Unit { name: "Japanese Yen", num: 2, den: 300 },
];
