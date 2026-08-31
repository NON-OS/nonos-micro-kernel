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

use ab_glyph::FontRef;

const FACES: &[&str] = &[
    "Inter-Regular.ttf",
    "Inter-Bold.ttf",
    "JetBrainsMono-Regular.ttf",
    "NotoSans-Regular.ttf",
    "NotoSans-Bold.ttf",
    "NotoSansMono-Regular.ttf",
];

fn font_path(name: &str) -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../toolkit/assets/fonts/").to_string() + name
}

#[test]
fn installed_faces_are_valid_and_distinct() {
    let mut bytes: Vec<(&str, Vec<u8>)> = Vec::new();
    for name in FACES {
        let path = font_path(name);
        let data = std::fs::read(&path).unwrap_or_else(|e| panic!("failed to read {name}: {e}"));
        assert!(FontRef::try_from_slice(&data).is_ok(), "{name} does not parse as a valid font");
        bytes.push((name, data));
    }
    for i in 0..bytes.len() {
        for j in (i + 1)..bytes.len() {
            assert_ne!(bytes[i].1, bytes[j].1, "{} and {} are byte-identical", bytes[i].0, bytes[j].0);
        }
    }
}
