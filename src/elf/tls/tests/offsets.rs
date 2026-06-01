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
fn test_calculate_tp_offset() {
    let info = TlsInfo::new(VirtAddr::new(0x1000), 128, 128, 16);
    assert_eq!(calculate_tp_offset(&info), 128);
}

#[test]
fn test_variable_offset() {
    let info = TlsInfo::new(VirtAddr::new(0x1000), 128, 128, 16);
    assert_eq!(variable_offset(&info, 64), -64);
}
