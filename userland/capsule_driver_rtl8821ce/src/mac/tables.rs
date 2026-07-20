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

//! The RTL8821CE MAC initialisation register program, as an operation table for
//! the executor. This reimplements rtw88 `rtw8821c_mac_init` (protocol, fast
//! EDCA, EDCA/timing, beacon and WMAC configuration) followed by the shared
//! `rtw_drv_info_cfg` tail. The register offsets and field values are rtw88
//! hardware facts (from `reg.h`, `bf.h` and the `WLAN_*`/`FAST_EDCA_*` defines in
//! `rtw8821c.h`); the composite 16/32-bit values are computed here and named in
//! comments so the program is traceable. The exact register writes are checked
//! against a modeled device in `rtl8821ce_proofs`.

use super::op::MacOp;

// rtw88 register offsets (reg.h unless noted). Named to match rtw88 exactly.
const REG_AMPDU_MAX_TIME_V1: u16 = 0x0455;
const REG_TX_HANG_CTRL: u16 = 0x045E;
const REG_PRECNT_CTRL: u16 = 0x04E5;
const REG_PROT_MODE_CTRL: u16 = 0x04C8;
const REG_BAR_MODE_CTRL: u16 = 0x04CC;
const REG_FAST_EDCA_VOVI_SETTING: u16 = 0x1448;
const REG_FAST_EDCA_BEBK_SETTING: u16 = 0x144C;
const REG_INIRTS_RATE_SEL: u16 = 0x0480; // rtw8821c.h
const REG_TIMER0_SRC_SEL: u16 = 0x05B4;
const REG_TXPAUSE: u16 = 0x0522;
const REG_SLOT: u16 = 0x051B;
const REG_PIFS: u16 = 0x0512;
const REG_SIFS: u16 = 0x0514;
const REG_EDCA_VO_PARAM: u16 = 0x0500;
const REG_EDCA_VI_PARAM: u16 = 0x0504;
const REG_RD_NAV_NXT: u16 = 0x0544;
const REG_RXTSF_OFFSET_CCK: u16 = 0x055E;
const REG_BCN_CTRL: u16 = 0x0550;
const REG_TBTT_PROHIBIT: u16 = 0x0540;
const REG_DRVERLYINT: u16 = 0x0558;
const REG_BCNDMATIM: u16 = 0x0559;
const REG_TX_PTCL_CTRL: u16 = 0x0520;
const REG_RXFLTMAP0: u16 = 0x06A0;
const REG_RXFLTMAP2: u16 = 0x06A4;
const REG_RCR: u16 = 0x0608;
const REG_RX_PKT_LIMIT: u16 = 0x060C;
const REG_TCR: u16 = 0x0604;
const REG_ACKTO_CCK: u16 = 0x0639;
const REG_WMAC_TRXPTCL_CTL_H: u16 = 0x066C;
const REG_SND_PTCL_CTRL: u16 = 0x0718; // bf.h
const REG_WMAC_OPTION_FUNCTION: u16 = 0x07D0;
const REG_RX_DRVINFO_SZ: u16 = 0x060F;
const REG_TRXFF_BNDY: u16 = 0x0114;

// Field bits and values (reg.h / bf.h / rtw8821c.h). Composites are computed
// here with the rtw88 expression they come from noted alongside.
const BIT_EN_EOF_V1: u8 = 1 << 2;
const BIT_EN_PRECNT: u16 = 1 << 11;
const BIT_INIRTS_RATE_SEL_5: u8 = 1 << 5;
const BIT_TSFT_SEL_TIMER0: u8 = (1 << 4) | (1 << 5) | (1 << 6);
const BIT_EN_BCN_FUNCTION: u8 = 1 << 3;
const BIT_SIFS_BK_EN_HI: u8 = 1 << 4; // BIT_SIFS_BK_EN (BIT12) >> 8
const BIT_WMAC_TRXPTCL_1: u8 = 1 << 1;
const BIT_DIS_CHK_VHTSIGB_CRC: u8 = 1 << 6;
const BIT_APP_PHYSTS: u32 = 1 << 28; // set into REG_RCR by rtw_drv_info_cfg
const WMAC_OPT_FUNC1_BITS: u32 = (1 << 8) | (1 << 9); // cleared at REG_WMAC_OPTION_FUNCTION+4

const WLAN_AMPDU_MAX_TIME: u8 = 0x70;
const WLAN_PRE_TXCNT_TIME_TH: u16 = 0x1E4;
// pre_txcnt = WLAN_PRE_TXCNT_TIME_TH | BIT_EN_PRECNT = 0x1E4 | 0x800 = 0x09E4.
const PRE_TXCNT: u16 = WLAN_PRE_TXCNT_TIME_TH | BIT_EN_PRECNT;
// value32 = RTS_LEN_TH(0xFF) | RTS_TX_TIME_TH(0x08)<<8 | MAX_AGG_PKT_LIMIT(0x20)<<16
//         | RTS_MAX_AGG_PKT_LIMIT(0x20)<<24 = 0x2020_08FF.
const PROT_MODE_CFG: u32 = 0xFF | (0x08 << 8) | (0x20 << 16) | (0x20 << 24);
// BAR_MODE+2 = BAR_RETRY_LIMIT(0x01) | RA_TRY_RATE_AGG_LIMIT(0x08)<<8 = 0x0801.
const BAR_MODE_HI: u16 = 0x01 | (0x08 << 8);
const FAST_EDCA_TH: u8 = 0x06; // VO/VI/BE/BK all 0x06
const WLAN_SLOT_TIME: u8 = 0x09;
const WLAN_PIFS_TIME: u8 = 0x19;
// SIFS = CCK_CTX(0x0A) | OFDM_CTX(0x0E)<<8 | CCK_TRX(0x10)<<16 | OFDM_TRX(0x10)<<24.
const WLAN_SIFS_CFG: u32 = 0x0A | (0x0E << 8) | (0x10 << 16) | (0x10 << 24);
const WLAN_VO_TXOP_LIMIT: u16 = 0x186;
const WLAN_VI_TXOP_LIMIT: u16 = 0x3BC;
// NAV = RDG_NAV(0x05) | TXOP_NAV(0x1B)<<16 = 0x001B_0005.
const WLAN_NAV_CFG: u32 = 0x05 | (0x1B << 16);
// RX_TSF = CCK_RX_TSF(0x30) | OFDM_RX_TSF(0x30)<<8 = 0x3030.
const WLAN_RX_TSF_CFG: u16 = 0x30 | (0x30 << 8);
// TBTT = TBTT_PROHIBIT(0x04) | TBTT_HOLD_TIME(0x64)<<8 = 0x6404.
const WLAN_TBTT_TIME: u32 = 0x04 | (0x64 << 8);
const WLAN_DRV_EARLY_INT: u8 = 0x04;
const WLAN_BCN_DMA_TIME: u8 = 0x02;
const WLAN_RX_FILTER0: u32 = 0x0FFF_FFFF;
const WLAN_RX_FILTER2: u16 = 0xFFFF;
const WLAN_RCR_CFG: u32 = 0xE400_220E;
// RXPKT_MAX_SZ_512 = WLAN_RXPKT_MAX_SZ(12288) >> 9 = 24 (0x18).
const WLAN_RXPKT_MAX_SZ_512: u8 = (12288u32 >> 9) as u8;
const WLAN_TX_FUNC_CFG1: u8 = 0x30;
const WLAN_TX_FUNC_CFG2: u8 = 0x30;
const WLAN_MAC_OPT_NORM_FUNC1: u8 = 0x98;
const WLAN_MAC_OPT_FUNC2: u32 = 0xB081_0041;
const PHY_STATUS_SIZE: u8 = 0x04; // rx.h PHY_STATUS_SIZE = 4
const ACKTO_CCK: u8 = 0x40;
const TRXFF_BNDY_LOW_NIBBLE: u8 = 0x0F; // value8 &= 0xF0; value8 |= 0xF

/// The RTL8821CE MAC init program, in rtw88 order: `rtw8821c_mac_init` then the
/// shared `rtw_drv_info_cfg` tail.
pub const MAC_INIT: &[MacOp] = &[
    // Protocol configuration.
    MacOp::write8(REG_AMPDU_MAX_TIME_V1, WLAN_AMPDU_MAX_TIME),
    MacOp::set8(REG_TX_HANG_CTRL, BIT_EN_EOF_V1),
    MacOp::write8(REG_PRECNT_CTRL, (PRE_TXCNT & 0xFF) as u8),
    MacOp::write8(REG_PRECNT_CTRL + 1, (PRE_TXCNT >> 8) as u8),
    MacOp::write32(REG_PROT_MODE_CTRL, PROT_MODE_CFG),
    MacOp::write16(REG_BAR_MODE_CTRL + 2, BAR_MODE_HI),
    MacOp::write8(REG_FAST_EDCA_VOVI_SETTING, FAST_EDCA_TH),
    MacOp::write8(REG_FAST_EDCA_VOVI_SETTING + 2, FAST_EDCA_TH),
    MacOp::write8(REG_FAST_EDCA_BEBK_SETTING, FAST_EDCA_TH),
    MacOp::write8(REG_FAST_EDCA_BEBK_SETTING + 2, FAST_EDCA_TH),
    MacOp::set8(REG_INIRTS_RATE_SEL, BIT_INIRTS_RATE_SEL_5),
    // EDCA configuration.
    MacOp::clear8(REG_TIMER0_SRC_SEL, BIT_TSFT_SEL_TIMER0),
    MacOp::write16(REG_TXPAUSE, 0),
    MacOp::write8(REG_SLOT, WLAN_SLOT_TIME),
    MacOp::write8(REG_PIFS, WLAN_PIFS_TIME),
    MacOp::write32(REG_SIFS, WLAN_SIFS_CFG),
    MacOp::write16(REG_EDCA_VO_PARAM + 2, WLAN_VO_TXOP_LIMIT),
    MacOp::write16(REG_EDCA_VI_PARAM + 2, WLAN_VI_TXOP_LIMIT),
    MacOp::write32(REG_RD_NAV_NXT, WLAN_NAV_CFG),
    MacOp::write16(REG_RXTSF_OFFSET_CCK, WLAN_RX_TSF_CFG),
    // Beacon control: enable TSF and related functions, then the send-beacon
    // timing registers.
    MacOp::set8(REG_BCN_CTRL, BIT_EN_BCN_FUNCTION),
    MacOp::write32(REG_TBTT_PROHIBIT, WLAN_TBTT_TIME),
    MacOp::write8(REG_DRVERLYINT, WLAN_DRV_EARLY_INT),
    MacOp::write8(REG_BCNDMATIM, WLAN_BCN_DMA_TIME),
    MacOp::clear8(REG_TX_PTCL_CTRL + 1, BIT_SIFS_BK_EN_HI),
    // WMAC configuration.
    MacOp::write32(REG_RXFLTMAP0, WLAN_RX_FILTER0),
    MacOp::write16(REG_RXFLTMAP2, WLAN_RX_FILTER2),
    MacOp::write32(REG_RCR, WLAN_RCR_CFG),
    MacOp::write8(REG_RX_PKT_LIMIT, WLAN_RXPKT_MAX_SZ_512),
    MacOp::write8(REG_TCR + 2, WLAN_TX_FUNC_CFG2),
    MacOp::write8(REG_TCR + 1, WLAN_TX_FUNC_CFG1),
    MacOp::write8(REG_ACKTO_CCK, ACKTO_CCK),
    MacOp::set8(REG_WMAC_TRXPTCL_CTL_H, BIT_WMAC_TRXPTCL_1),
    MacOp::set8(REG_SND_PTCL_CTRL, BIT_DIS_CHK_VHTSIGB_CRC),
    MacOp::write32(REG_WMAC_OPTION_FUNCTION + 8, WLAN_MAC_OPT_FUNC2),
    MacOp::write8(REG_WMAC_OPTION_FUNCTION + 4, WLAN_MAC_OPT_NORM_FUNC1),
    // rtw_drv_info_cfg tail (11ac path): RX drvinfo size, the RX FIFO boundary
    // low-nibble fix (clear then set 0xF), then RCR app-physts and the option
    // function-1 clear.
    MacOp::write8(REG_RX_DRVINFO_SZ, PHY_STATUS_SIZE),
    MacOp::clear8(REG_TRXFF_BNDY + 1, TRXFF_BNDY_LOW_NIBBLE),
    MacOp::set8(REG_TRXFF_BNDY + 1, TRXFF_BNDY_LOW_NIBBLE),
    MacOp::set32(REG_RCR, BIT_APP_PHYSTS),
    MacOp::clear32(REG_WMAC_OPTION_FUNCTION + 4, WMAC_OPT_FUNC1_BITS),
];
