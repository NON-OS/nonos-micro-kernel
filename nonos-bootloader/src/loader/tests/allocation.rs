// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::loader::*;

#[test]
fn test_allocation_record() {
    let record = memory::AllocationRecord::new(0x100000, 10);
    assert!(record.is_valid());
    assert_eq!(record.size_bytes(), 10 * types::memory::PAGE_SIZE);

    let invalid = memory::AllocationRecord::default();
    assert!(!invalid.is_valid());
}

#[test]
fn test_memory_region() {
    let region =
        memory::MemoryRegion { start: 0x100000, size: 0x10000, writable: true, executable: false };

    assert!(region.contains(0x100000));
    assert!(region.contains(0x10FFFF));
    assert!(!region.contains(0x110000));
    assert_eq!(region.end(), 0x110000);
}
