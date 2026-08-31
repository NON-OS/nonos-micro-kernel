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

//! The Insert > Special Character table: the label a row shows and the UTF-8
//! text it inserts. The row index is the selector, so order is load-bearing.

pub(in crate::editor) const SPECIALS: [(&str, &str); 14] = [
    ("—   Em Dash", "—"),
    ("–   En Dash", "–"),
    ("…   Ellipsis", "…"),
    ("•   Bullet", "•"),
    ("→   Right Arrow", "→"),
    ("←   Left Arrow", "←"),
    ("×   Multiplication", "×"),
    ("÷   Division", "÷"),
    ("±   Plus-Minus", "±"),
    ("≠   Not Equal", "≠"),
    ("°   Degree", "°"),
    ("€   Euro", "€"),
    ("£   Pound", "£"),
    ("π   Pi", "π"),
];
