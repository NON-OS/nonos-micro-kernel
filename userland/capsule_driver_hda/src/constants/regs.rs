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

pub const GCAP: u32 = 0x00;
pub const VMIN: u32 = 0x02;
pub const VMAJ: u32 = 0x03;
pub const OUTPAY: u32 = 0x04;
pub const INPAY: u32 = 0x06;
pub const GCTL: u32 = 0x08;
pub const STATESTS: u32 = 0x0e;
pub const GSTS: u32 = 0x10;
pub const INTCTL: u32 = 0x20;
pub const INTSTS: u32 = 0x24;
pub const IC: u32 = 0x60;
pub const IR: u32 = 0x64;
pub const IRS: u32 = 0x68;

pub const GCTL_CRST: u32 = 1 << 0;
pub const IRS_BUSY: u8 = 1 << 0;
pub const IRS_VALID: u8 = 1 << 1;
pub const VERB_GET_PARAMETER: u16 = 0x0f00;
pub const VERB_GET_CONNECT_LIST: u16 = 0x0f02;
pub const PARAM_VENDOR_ID: u16 = 0x00;
pub const PARAM_SUBNODE_COUNT: u16 = 0x04;
pub const PARAM_FUNCTION_GROUP_TYPE: u16 = 0x05;
pub const PARAM_AUDIO_WIDGET_CAP: u16 = 0x09;
pub const FUNCTION_GROUP_AUDIO: u32 = 0x01;
pub const WIDGET_TYPE_DAC: u32 = 0x0;
pub const WIDGET_TYPE_PIN: u32 = 0x4;

pub const CORBLBASE: u32 = 0x40;
pub const CORBUBASE: u32 = 0x44;
pub const CORBWP: u32 = 0x48;
pub const CORBRP: u32 = 0x4a;
pub const CORBCTL: u32 = 0x4c;
pub const CORBSTS: u32 = 0x4d;
pub const CORBSIZE: u32 = 0x4e;
pub const RIRBLBASE: u32 = 0x50;
pub const RIRBUBASE: u32 = 0x54;
pub const RIRBWP: u32 = 0x58;
pub const RINTCNT: u32 = 0x5a;
pub const RIRBCTL: u32 = 0x5c;
pub const RIRBSTS: u32 = 0x5d;
pub const RIRBSIZE: u32 = 0x5e;

pub const CORBCTL_RUN: u8 = 1 << 1;
pub const CORBCTL_CMEIE: u8 = 1 << 0;
pub const CORBRP_RST: u16 = 1 << 15;
pub const CORBSIZE_256: u8 = 0x02;
pub const RIRBCTL_DMAEN: u8 = 1 << 1;
pub const RIRBCTL_RINTCTL: u8 = 1 << 0;
pub const RIRBWP_RST: u16 = 1 << 15;
pub const RIRBSIZE_256: u8 = 0x02;
pub const RINTCNT_ONE: u16 = 1;
