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

use super::super::AslrManager;

#[test]
fn test_random_offset_zero_max() {
    assert_eq!(AslrManager::new().random_offset(0), 0);
}

#[test]
fn test_random_offset_within_range() {
    let mut manager = AslrManager::new();
    for _ in 0..100 {
        assert!(manager.random_offset(1000) < 1000);
    }
}

#[test]
fn test_randomize_base_disabled() {
    assert_eq!(AslrManager::disabled().randomize_base(0x400000), 0x400000);
}

#[test]
fn test_randomize_base_page_aligned() {
    assert_eq!(AslrManager::new().randomize_base(0x400000) & 0xFFF, 0);
}

#[test]
fn test_randomize_stack_disabled() {
    assert_eq!(AslrManager::disabled().randomize_stack(0x7FFF_FFFF_E000), 0x7FFF_FFFF_E000);
}

#[test]
fn test_randomize_stack_below_base() {
    let result = AslrManager::new().randomize_stack(0x7FFF_FFFF_E000);
    assert!(result <= 0x7FFF_FFFF_E000);
    assert_eq!(result & 0xFFF, 0);
}

#[test]
fn test_randomize_heap_disabled() {
    assert_eq!(AslrManager::disabled().randomize_heap(0x1000_0000), 0x1000_0000);
}

#[test]
fn test_randomize_heap_above_base() {
    let result = AslrManager::new().randomize_heap(0x1000_0000);
    assert!(result >= 0x1000_0000);
    assert_eq!(result & 0xFFF, 0);
}

#[test]
fn test_reseed() {
    let mut manager = AslrManager::new();
    manager.reseed();
    let _entropy = manager.entropy();
}
