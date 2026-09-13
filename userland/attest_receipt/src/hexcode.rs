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

//! Roots as people pass them around.
//!
//! A root travels by being read off a screen and typed somewhere else, so the
//! reader accepts it the way it is written and refuses everything else. A
//! short root is not padded and a long one is not cut: either would compare a
//! value the person did not mean against the set they did mean, and report a
//! mismatch that reads as a bad machine rather than a typo.

/// Lowercase hex, no separators.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// A thirty-two byte root from its hex text, with an optional `0x`.
pub fn parse_root(s: &str) -> Option<[u8; 32]> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(s.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::{hex, parse_root};

    #[test]
    fn a_root_survives_the_round_trip() {
        let root = [0x91u8; 32];
        assert_eq!(parse_root(&hex(&root)), Some(root));
        assert_eq!(parse_root(&format!("0x{}", hex(&root))), Some(root));
    }

    #[test]
    fn a_root_of_the_wrong_length_is_refused_rather_than_padded() {
        assert_eq!(parse_root("91cd5556"), None);
        assert_eq!(parse_root(&"a".repeat(65)), None);
    }

    #[test]
    fn a_non_hex_character_is_refused() {
        /*
         * `from_str_radix` on "zz" fails rather than yielding zero, which is
         * what keeps a mistyped root from silently becoming a valid looking
         * one that simply does not match.
         */
        assert_eq!(parse_root(&"z".repeat(64)), None);
    }
}
