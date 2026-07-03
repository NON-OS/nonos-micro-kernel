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

pub struct Shortcut {
    pub label: &'static [u8],
    pub url: &'static str,
    pub badge: &'static [u8],
    pub color: u32,
}

pub const SHORTCUTS: [Shortcut; 4] = [
    Shortcut {
        label: b"neverssl.com",
        url: "http://neverssl.com/",
        badge: b"N",
        color: 0xFF3F_B950,
    },
    Shortcut { label: b"example.com", url: "http://example.com/", badge: b"E", color: 0xFF38_8BFD },
    Shortcut {
        label: b"info.cern.ch",
        url: "http://info.cern.ch/",
        badge: b"C",
        color: 0xFFE5_534B,
    },
    Shortcut {
        label: b"httpforever.com",
        url: "http://httpforever.com/",
        badge: b"H",
        color: 0xFFA3_71F7,
    },
];
