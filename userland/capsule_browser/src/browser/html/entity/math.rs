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

/// Arrows and the mathematics that turns up in ordinary prose.
pub fn math(name: &str) -> Option<&'static str> {
    Some(match name {
        "larr" | "leftarrow" | "LeftArrow" => "\u{2190}",
        "uarr" | "uparrow" => "\u{2191}",
        "rarr" | "rightarrow" | "RightArrow" => "\u{2192}",
        "darr" | "downarrow" => "\u{2193}",
        "harr" | "leftrightarrow" => "\u{2194}",
        "lArr" | "Leftarrow" => "\u{21D0}",
        "rArr" | "Rightarrow" => "\u{21D2}",
        "hArr" | "Leftrightarrow" => "\u{21D4}",
        "ne" | "NotEqual" => "\u{2260}",
        "le" | "leq" => "\u{2264}",
        "ge" | "geq" => "\u{2265}",
        "asymp" => "\u{2248}",
        "equiv" => "\u{2261}",
        "infin" => "\u{221E}",
        "radic" | "Sqrt" => "\u{221A}",
        "sum" => "\u{2211}",
        "prod" => "\u{220F}",
        "int" => "\u{222B}",
        "part" => "\u{2202}",
        "nabla" => "\u{2207}",
        "forall" => "\u{2200}",
        "exist" => "\u{2203}",
        "empty" | "emptyset" => "\u{2205}",
        "isin" | "in" => "\u{2208}",
        "notin" => "\u{2209}",
        "cap" => "\u{2229}",
        "cup" => "\u{222A}",
        "sub" => "\u{2282}",
        "sup" => "\u{2283}",
        "and" => "\u{2227}",
        "or" => "\u{2228}",
        "lowast" => "\u{2217}",
        "prop" => "\u{221D}",
        "ang" => "\u{2220}",
        _ => return None,
    })
}
