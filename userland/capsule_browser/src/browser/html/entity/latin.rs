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

/// Accented Latin letters, lower case.
///
/// Any page not written in English reaches for these, and one that arrives
/// undecoded prints its own source at the reader: `Fran&ccedil;ais` rather
/// than the word.
pub fn latin_lower(name: &str) -> Option<&'static str> {
    Some(match name {
        "agrave" => "\u{00E0}",
        "aacute" => "\u{00E1}",
        "acirc" => "\u{00E2}",
        "atilde" => "\u{00E3}",
        "auml" => "\u{00E4}",
        "aring" => "\u{00E5}",
        "aelig" => "\u{00E6}",
        "ccedil" => "\u{00E7}",
        "egrave" => "\u{00E8}",
        "eacute" => "\u{00E9}",
        "ecirc" => "\u{00EA}",
        "euml" => "\u{00EB}",
        "igrave" => "\u{00EC}",
        "iacute" => "\u{00ED}",
        "icirc" => "\u{00EE}",
        "iuml" => "\u{00EF}",
        "eth" => "\u{00F0}",
        "ntilde" => "\u{00F1}",
        "ograve" => "\u{00F2}",
        "oacute" => "\u{00F3}",
        "ocirc" => "\u{00F4}",
        "otilde" => "\u{00F5}",
        "ouml" => "\u{00F6}",
        "oslash" => "\u{00F8}",
        "ugrave" => "\u{00F9}",
        "uacute" => "\u{00FA}",
        "ucirc" => "\u{00FB}",
        "uuml" => "\u{00FC}",
        "yacute" => "\u{00FD}",
        "thorn" => "\u{00FE}",
        "yuml" => "\u{00FF}",
        "szlig" => "\u{00DF}",
        "oelig" => "\u{0153}",
        "scaron" => "\u{0161}",
        _ => return None,
    })
}

/// Accented Latin letters, upper case.
pub fn latin_upper(name: &str) -> Option<&'static str> {
    Some(match name {
        "Agrave" => "\u{00C0}",
        "Aacute" => "\u{00C1}",
        "Acirc" => "\u{00C2}",
        "Atilde" => "\u{00C3}",
        "Auml" => "\u{00C4}",
        "Aring" => "\u{00C5}",
        "AElig" => "\u{00C6}",
        "Ccedil" => "\u{00C7}",
        "Egrave" => "\u{00C8}",
        "Eacute" => "\u{00C9}",
        "Ecirc" => "\u{00CA}",
        "Euml" => "\u{00CB}",
        "Igrave" => "\u{00CC}",
        "Iacute" => "\u{00CD}",
        "Icirc" => "\u{00CE}",
        "Iuml" => "\u{00CF}",
        "ETH" => "\u{00D0}",
        "Ntilde" => "\u{00D1}",
        "Ograve" => "\u{00D2}",
        "Oacute" => "\u{00D3}",
        "Ocirc" => "\u{00D4}",
        "Otilde" => "\u{00D5}",
        "Ouml" => "\u{00D6}",
        "Oslash" => "\u{00D8}",
        "Ugrave" => "\u{00D9}",
        "Uacute" => "\u{00DA}",
        "Ucirc" => "\u{00DB}",
        "Uuml" => "\u{00DC}",
        "Yacute" => "\u{00DD}",
        "THORN" => "\u{00DE}",
        "OElig" => "\u{0152}",
        "Scaron" => "\u{0160}",
        "Yuml" => "\u{0178}",
        _ => return None,
    })
}
