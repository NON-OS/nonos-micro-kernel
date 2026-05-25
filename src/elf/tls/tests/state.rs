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

use super::super::*;
use crate::memory::addr::VirtAddr;

#[test]
fn test_tls_info_new() {
    let info = TlsInfo::new(VirtAddr::new(0x1000), 256, 512, 16);
    assert_eq!(info.template_addr, VirtAddr::new(0x1000));
    assert_eq!(info.template_size, 256);
    assert_eq!(info.memory_size, 512);
    assert_eq!(info.alignment, 16);
}

#[test]
fn test_tls_info_bss_size() {
    let info = TlsInfo::new(VirtAddr::new(0x1000), 256, 512, 16);
    let other = TlsInfo::new(VirtAddr::new(0x1000), 256, 256, 16);
    assert_eq!(info.bss_size(), 256);
    assert_eq!(other.bss_size(), 0);
}

#[test]
fn test_tls_info_has_bss() {
    assert!(TlsInfo::new(VirtAddr::new(0x1000), 256, 512, 16).has_bss());
    assert!(!TlsInfo::new(VirtAddr::new(0x1000), 256, 256, 16).has_bss());
}

#[test]
fn test_tls_info_is_empty() {
    assert!(TlsInfo::new(VirtAddr::new(0x1000), 0, 0, 16).is_empty());
    assert!(!TlsInfo::new(VirtAddr::new(0x1000), 256, 256, 16).is_empty());
}

#[test]
fn test_tls_info_default() {
    let info = TlsInfo::default();
    assert_eq!(info.template_addr, VirtAddr::new(0));
    assert_eq!(info.template_size, 0);
    assert_eq!(info.memory_size, 0);
    assert_eq!(info.alignment, DEFAULT_TLS_ALIGNMENT);
}

#[test]
fn test_constants() {
    assert_eq!(DEFAULT_TLS_ALIGNMENT, 16);
    assert_eq!(TCB_SIZE, 16);
}
