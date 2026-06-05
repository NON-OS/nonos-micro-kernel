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

use crate::catalog::entry::Entry;

pub(super) const FIELD_FOCUS: &[Entry] = &[
    Entry { slug: b"field-focus-1", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-1.jpg") },
    Entry { slug: b"field-focus-2", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-2.jpg") },
    Entry { slug: b"field-focus-3", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-3.jpg") },
    Entry { slug: b"field-focus-4", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-4.jpg") },
    Entry { slug: b"field-focus-5", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-5.jpg") },
    Entry { slug: b"field-focus-6", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-6.jpg") },
    Entry { slug: b"field-focus-7", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-7.jpg") },
    Entry { slug: b"field-focus-8", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-8.jpg") },
    Entry { slug: b"field-focus-9", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-9.jpg") },
    Entry { slug: b"field-focus-10", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-10.jpg") },
    Entry { slug: b"field-focus-11", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-11.jpg") },
    Entry { slug: b"field-focus-12", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-12.jpg") },
    Entry { slug: b"field-focus-13", bytes: include_bytes!("../../../../../nonos-data/wallpapers/field-focus-13.jpg") },
];
