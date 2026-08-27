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

mod fault;
mod global;
mod invalidate;
mod timing;

pub use fault::{frcd_reason, frcd_source, FRCD_FAULT, FRCD_TYPE_READ, FSTS_PFO, FSTS_PPF};
pub use global::{
    CAP, ECAP, FECTL, FSTS, GCMD, GCMD_SRTP, GCMD_TE, GCMD_WBF, GSTS, GSTS_RTPS, GSTS_TES,
    GSTS_WBFS, RTADDR, VER,
};
pub use invalidate::{
    iotlb_offset, iva_offset, CCMD, CCMD_CIRG_GLOBAL, CCMD_ICC, IOTLB_IAIG_MASK,
    IOTLB_IIRG_GLOBAL, IOTLB_IVT,
};
pub use timing::COMMAND_SPINS;
