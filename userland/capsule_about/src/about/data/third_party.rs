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


// Every row is checked against a licence file in the tree: third_party/*/LICENSE,
// toolkit/assets/fonts/*-OFL.txt, and the crate manifests under ~/.cargo for ab_glyph.
// A component with no licence file in the tree does not belong in this table.

pub struct Component {
    pub name: &'static [u8],
    pub role: &'static [u8],
    pub license: &'static [u8],
}

pub const COMPONENTS: [Component; 7] = [
    Component {
        name: b"PQClean ML-DSA-65",
        role: b"capsule signatures",
        license: b"CC0-1.0",
    },
    Component {
        name: b"PQClean ML-KEM",
        role: b"key encapsulation",
        license: b"CC0-1.0",
    },
    Component {
        name: b"ab_glyph",
        role: b"glyph rasteriser",
        license: b"Apache-2.0",
    },
    Component {
        name: b"Noto Sans",
        role: b"interface typeface",
        license: b"OFL-1.1",
    },
    Component {
        name: b"Noto Sans Mono",
        role: b"data typeface",
        license: b"OFL-1.1",
    },
    Component {
        name: b"minimp3",
        role: b"audio decode",
        license: b"CC0-1.0",
    },
    Component {
        name: b"relibc (Redox)",
        role: b"C runtime graft",
        license: b"MIT",
    },
];
