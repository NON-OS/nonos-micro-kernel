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
fn test_tls_info_effective_alignment() {
    assert_eq!(
        TlsInfo::new(VirtAddr::new(0x1000), 256, 256, 8).effective_alignment(),
        DEFAULT_TLS_ALIGNMENT
    );
    assert_eq!(TlsInfo::new(VirtAddr::new(0x1000), 256, 256, 64).effective_alignment(), 64);
}

#[test]
fn test_tls_info_allocation_size() {
    assert_eq!(TlsInfo::new(VirtAddr::new(0x1000), 100, 100, 16).allocation_size(), 112);
    assert_eq!(TlsInfo::new(VirtAddr::new(0x1000), 128, 128, 16).allocation_size(), 128);
}

#[test]
fn test_tls_info_total_size_with_tcb() {
    assert_eq!(
        TlsInfo::new(VirtAddr::new(0x1000), 128, 128, 16).total_size_with_tcb(),
        128 + TCB_SIZE
    );
}

#[test]
fn test_tls_info_template_end() {
    assert_eq!(
        TlsInfo::new(VirtAddr::new(0x1000), 256, 512, 16).template_end(),
        VirtAddr::new(0x1100)
    );
}

#[test]
fn test_tls_info_zero_alignment() {
    assert_eq!(TlsInfo::new(VirtAddr::new(0x1000), 256, 256, 0).alignment, 1);
}
