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

use crate::arch::x86_64::acpi::aml::crs::{
    resource_bytes, scan_large_descriptors, walk_large_descriptors,
};
use crate::arch::x86_64::acpi::aml::types::LpssController;

use super::apply::apply_descriptor;

/// Fill an `LpssController` from its `_CRS` ResourceTemplate, preferring the
/// length-delimited descriptor walk and falling back to a raw byte scan when
/// the wrapper offset is unusual.
pub fn parse_controller_crs(crs_value: &[u8], ctl: &mut LpssController) {
    if let Some(bytes) = resource_bytes(crs_value) {
        walk_large_descriptors(bytes, |lead, data| apply_descriptor(lead, data, ctl));
    }
    if ctl.mmio_base == 0 || !ctl.has_irq {
        scan_large_descriptors(crs_value, |lead, data| apply_descriptor(lead, data, ctl));
    }
}
