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

pub(super) const NETWORK_TOPOLOGY: &[Entry] = &[
    Entry { slug: b"network-topology-1", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-1.jpg") },
    Entry { slug: b"network-topology-2", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-2.jpg") },
    Entry { slug: b"network-topology-3", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-3.jpg") },
    Entry { slug: b"network-topology-4", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-4.jpg") },
    Entry { slug: b"network-topology-5", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-5.jpg") },
    Entry { slug: b"network-topology-6", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-6.jpg") },
    Entry { slug: b"network-topology-7", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-7.jpg") },
    Entry { slug: b"network-topology-8", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-8.jpg") },
    Entry { slug: b"network-topology-9", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-9.jpg") },
    Entry { slug: b"network-topology-10", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-10.jpg") },
    Entry { slug: b"network-topology-11", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-11.jpg") },
    Entry { slug: b"network-topology-13", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-13.jpg") },
    Entry { slug: b"network-topology-14", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-14.jpg") },
    Entry { slug: b"network-topology-15", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-15.jpg") },
    Entry { slug: b"network-topology-16", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-16.jpg") },
    Entry { slug: b"network-topology-17", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-17.jpg") },
    Entry { slug: b"network-topology-18", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-18.jpg") },
    Entry { slug: b"network-topology-19", bytes: include_bytes!("../../../../../nonos-data/wallpapers/network-topology-19.jpg") },
];
