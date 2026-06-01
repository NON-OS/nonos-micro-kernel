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

use super::entry::Entry;

pub const ENTRIES: &[Entry] = &[
    Entry { slug: b"field-focus-1",  bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-1.jpg") },
    Entry { slug: b"field-focus-2",  bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-2.jpg") },
    Entry { slug: b"field-focus-3",  bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-3.jpg") },
    Entry { slug: b"field-focus-4",  bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-4.jpg") },
    Entry { slug: b"field-focus-5",  bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-5.jpg") },
    Entry { slug: b"field-focus-6",  bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-6.jpg") },
    Entry { slug: b"field-focus-7",  bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-7.jpg") },
    Entry { slug: b"field-focus-8",  bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-8.jpg") },
    Entry { slug: b"field-focus-9",  bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-9.jpg") },
    Entry { slug: b"field-focus-10", bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-10.jpg") },
    Entry { slug: b"field-focus-11", bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-11.jpg") },
    Entry { slug: b"field-focus-12", bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-12.jpg") },
    Entry { slug: b"field-focus-13", bytes: include_bytes!("../../../../nonos-data/wallpapers/field-focus-13.jpg") },
    Entry { slug: b"hardware-aesthetic-1",  bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-1.jpg") },
    Entry { slug: b"hardware-aesthetic-2",  bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-2.jpg") },
    Entry { slug: b"hardware-aesthetic-3",  bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-3.jpg") },
    Entry { slug: b"hardware-aesthetic-4",  bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-4.jpg") },
    Entry { slug: b"hardware-aesthetic-5",  bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-5.jpg") },
    Entry { slug: b"hardware-aesthetic-6",  bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-6.jpg") },
    Entry { slug: b"hardware-aesthetic-7",  bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-7.jpg") },
    Entry { slug: b"hardware-aesthetic-8",  bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-8.jpg") },
    Entry { slug: b"hardware-aesthetic-9",  bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-9.jpg") },
    Entry { slug: b"hardware-aesthetic-10", bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-10.jpg") },
    Entry { slug: b"hardware-aesthetic-11", bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-11.jpg") },
    Entry { slug: b"hardware-aesthetic-12", bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-12.jpg") },
    Entry { slug: b"hardware-aesthetic-13", bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-13.jpg") },
    Entry { slug: b"hardware-aesthetic-14", bytes: include_bytes!("../../../../nonos-data/wallpapers/hardware-aesthetic-14.jpg") },
    Entry { slug: b"network-topology-1",  bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-1.jpg") },
    Entry { slug: b"network-topology-2",  bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-2.jpg") },
    Entry { slug: b"network-topology-3",  bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-3.jpg") },
    Entry { slug: b"network-topology-4",  bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-4.jpg") },
    Entry { slug: b"network-topology-5",  bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-5.jpg") },
    Entry { slug: b"network-topology-6",  bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-6.jpg") },
    Entry { slug: b"network-topology-7",  bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-7.jpg") },
    Entry { slug: b"network-topology-8",  bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-8.jpg") },
    Entry { slug: b"network-topology-9",  bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-9.jpg") },
    Entry { slug: b"network-topology-10", bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-10.jpg") },
    Entry { slug: b"network-topology-11", bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-11.jpg") },
    Entry { slug: b"network-topology-13", bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-13.jpg") },
    Entry { slug: b"network-topology-14", bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-14.jpg") },
    Entry { slug: b"network-topology-15", bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-15.jpg") },
    Entry { slug: b"network-topology-16", bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-16.jpg") },
    Entry { slug: b"network-topology-17", bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-17.jpg") },
    Entry { slug: b"network-topology-18", bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-18.jpg") },
    Entry { slug: b"network-topology-19", bytes: include_bytes!("../../../../nonos-data/wallpapers/network-topology-19.jpg") },
    Entry { slug: b"special-variant-1a", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-1a.jpg") },
    Entry { slug: b"special-variant-1b", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-1b.jpg") },
    Entry { slug: b"special-variant-2a", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-2a.jpg") },
    Entry { slug: b"special-variant-2b", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-2b.jpg") },
    Entry { slug: b"special-variant-3",  bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-3.jpg") },
    Entry { slug: b"special-variant-4",  bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-4.jpg") },
    Entry { slug: b"special-variant-5",  bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-5.jpg") },
    Entry { slug: b"special-variant-6",  bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-6.jpg") },
    Entry { slug: b"special-variant-7",  bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-7.jpg") },
    Entry { slug: b"special-variant-8",  bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-8.jpg") },
    Entry { slug: b"special-variant-9",  bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-9.jpg") },
    Entry { slug: b"special-variant-10", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-10.jpg") },
    Entry { slug: b"special-variant-11", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-11.jpg") },
    Entry { slug: b"special-variant-12", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-12.jpg") },
    Entry { slug: b"special-variant-13", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-13.jpg") },
    Entry { slug: b"special-variant-14", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-14.jpg") },
    Entry { slug: b"special-variant-15", bytes: include_bytes!("../../../../nonos-data/wallpapers/special-variant-15.jpg") },
];
