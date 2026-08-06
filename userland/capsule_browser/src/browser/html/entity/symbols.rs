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

/// Marks, currency and the arithmetic signs pages actually print.
pub fn symbols(name: &str) -> Option<&'static str> {
    Some(match name {
        "copy" | "COPY" => "\u{00A9}",
        "reg" | "REG" | "circledR" => "\u{00AE}",
        "trade" => "\u{2122}",
        "deg" => "\u{00B0}",
        "times" => "\u{00D7}",
        "divide" | "div" => "\u{00F7}",
        "plusmn" | "pm" => "\u{00B1}",
        "minus" => "\u{2212}",
        "micro" => "\u{00B5}",
        "not" => "\u{00AC}",
        "sup1" => "\u{00B9}",
        "sup2" => "\u{00B2}",
        "sup3" => "\u{00B3}",
        "frac14" => "\u{00BC}",
        "frac12" | "half" => "\u{00BD}",
        "frac34" => "\u{00BE}",
        "euro" => "\u{20AC}",
        "pound" => "\u{00A3}",
        "yen" => "\u{00A5}",
        "cent" => "\u{00A2}",
        "curren" => "\u{00A4}",
        "dollar" => "$",
        "permil" => "\u{2030}",
        "starf" | "star" => "\u{2605}",
        "check" | "checkmark" => "\u{2713}",
        "cross" => "\u{2717}",
        "hearts" | "heartsuit" => "\u{2665}",
        "diams" | "diamondsuit" => "\u{2666}",
        "clubs" | "clubsuit" => "\u{2663}",
        "spades" | "spadesuit" => "\u{2660}",
        _ => return None,
    })
}
