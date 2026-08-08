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

/// Dashes, quotation marks and the punctuation that carries typography.
///
/// These were folded onto ASCII lookalikes, so an em dash arrived as two
/// hyphens and a curly quote as a straight one. The bundled face has all of
/// them, and a page that asked for one and got two characters instead has
/// had its line lengths changed by the reader.
pub fn punctuation(name: &str) -> Option<&'static str> {
    Some(match name {
        "ndash" => "\u{2013}",
        "mdash" => "\u{2014}",
        "horbar" => "\u{2015}",
        "lsquo" => "\u{2018}",
        "rsquo" | "rsquor" => "\u{2019}",
        "sbquo" => "\u{201A}",
        "ldquo" => "\u{201C}",
        "rdquo" | "rdquor" => "\u{201D}",
        "bdquo" => "\u{201E}",
        "hellip" | "mldr" => "\u{2026}",
        "bull" | "bullet" => "\u{2022}",
        "middot" | "centerdot" => "\u{00B7}",
        "dagger" => "\u{2020}",
        "Dagger" => "\u{2021}",
        "prime" => "\u{2032}",
        "Prime" => "\u{2033}",
        "laquo" => "\u{00AB}",
        "raquo" => "\u{00BB}",
        "lsaquo" => "\u{2039}",
        "rsaquo" => "\u{203A}",
        "iexcl" => "\u{00A1}",
        "iquest" => "\u{00BF}",
        "sect" => "\u{00A7}",
        "para" => "\u{00B6}",
        "brvbar" => "\u{00A6}",
        "uml" | "die" => "\u{00A8}",
        "ordf" => "\u{00AA}",
        "ordm" => "\u{00BA}",
        "macr" => "\u{00AF}",
        "acute" => "\u{00B4}",
        "cedil" => "\u{00B8}",
        _ => return None,
    })
}
