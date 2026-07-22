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

pub mod pci;
pub mod regs;

pub use pci::{CLASS_AUDIO, HDA_BAR_INDEX, HDA_BAR_MIN_SIZE};
pub use regs::{
    GCAP, GCTL, GCTL_CRST, GSTS, IC, INPAY, INTCTL, INTSTS, IR, IRS, IRS_BUSY, IRS_VALID, OUTPAY,
    PARAM_VENDOR_ID, STATESTS, VERB_GET_PARAMETER,
};
pub use regs::{VMAJ, VMIN};
pub use regs::{
    CORBCTL, CORBCTL_CMEIE, CORBCTL_RUN, CORBLBASE, CORBRP, CORBRP_RST, CORBSIZE, CORBSIZE_256,
    CORBSTS, CORBUBASE, CORBWP, RINTCNT, RINTCNT_ONE, RIRBCTL, RIRBCTL_DMAEN, RIRBCTL_RINTCTL,
    RIRBLBASE, RIRBSIZE, RIRBSIZE_256, RIRBSTS, RIRBUBASE, RIRBWP, RIRBWP_RST,
};
