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

pub(super) const SPECIAL_VARIANT: &[Entry] = &[
    Entry { slug: b"special-variant-1a", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-1a.jpg") },
    Entry { slug: b"special-variant-1b", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-1b.jpg") },
    Entry { slug: b"special-variant-2a", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-2a.jpg") },
    Entry { slug: b"special-variant-2b", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-2b.jpg") },
    Entry { slug: b"special-variant-3", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-3.jpg") },
    Entry { slug: b"special-variant-4", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-4.jpg") },
    Entry { slug: b"special-variant-5", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-5.jpg") },
    Entry { slug: b"special-variant-6", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-6.jpg") },
    Entry { slug: b"special-variant-7", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-7.jpg") },
    Entry { slug: b"special-variant-8", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-8.jpg") },
    Entry { slug: b"special-variant-9", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-9.jpg") },
    Entry { slug: b"special-variant-10", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-10.jpg") },
    Entry { slug: b"special-variant-11", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-11.jpg") },
    Entry { slug: b"special-variant-12", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-12.jpg") },
    Entry { slug: b"special-variant-13", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-13.jpg") },
    Entry { slug: b"special-variant-14", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-14.jpg") },
    Entry { slug: b"special-variant-15", bytes: include_bytes!("../../../../../nonos-data/wallpapers/special-variant-15.jpg") },
];
