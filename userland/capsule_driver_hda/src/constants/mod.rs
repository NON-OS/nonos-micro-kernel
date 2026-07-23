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
    CORBCTL, CORBCTL_CMEIE, CORBCTL_RUN, CORBLBASE, CORBRP, CORBRP_RST, CORBSIZE, CORBSIZE_256,
    CORBSTS, CORBUBASE, CORBWP, RINTCNT, RINTCNT_ONE, RIRBCTL, RIRBCTL_DMAEN, RIRBCTL_RINTCTL,
    RIRBLBASE, RIRBSIZE, RIRBSIZE_256, RIRBSTS, RIRBSTS_INTFL, RIRBUBASE, RIRBWP, RIRBWP_RST,
};
pub use regs::{
    FUNCTION_GROUP_AUDIO, PARAM_AUDIO_WIDGET_CAP, PARAM_FUNCTION_GROUP_TYPE, PARAM_SUBNODE_COUNT,
    VERB_GET_CONNECT_LIST, WIDGET_TYPE_DAC, WIDGET_TYPE_PIN,
};
pub use regs::{
    GCAP, GCTL, GCTL_CRST, GSTS, IC, INPAY, INTCTL, INTSTS, IR, IRS, IRS_BUSY, IRS_VALID, OUTPAY,
    PARAM_VENDOR_ID, STATESTS, VERB_GET_PARAMETER,
};
pub use regs::{VMAJ, VMIN};
pub use regs::{
    AMP_OUT_UNMUTE, INTCTL_GIE, PIN_OUT_ENABLE, POWER_D0, SDCTL_IOCE, SDCTL_RUN, SDCTL_SRST,
    SDSTS_BCIS, SD_BDPL, SD_BDPU, SD_CBL, SD_CTL, SD_FMT, SD_LPIB, SD_LVI, SD_STS,
    STREAM_FMT_48K16S, STREAM_TAG, VERB_SET_AMP_GAIN_MUTE, VERB_SET_CHANNEL_STREAMID,
    VERB_SET_PIN_WIDGET_CONTROL, VERB_SET_POWER_STATE, VERB_SET_STREAM_FORMAT,
};
