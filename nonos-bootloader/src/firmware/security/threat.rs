// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}
#[derive(Debug, Clone)]
pub struct ThreatAnalysis {
    pub level: ThreatLevel,
    pub indicators: u32,
    pub confidence: u8,
    pub malware_signatures: u8,
}

pub fn detect_threats(data: &[u8], vendor_id: u16) -> ThreatAnalysis {
    let mut ind = 0u32;
    let mut mal = 0u8;
    if has_suspicious(data) {
        ind += 1;
    }
    if high_entropy(data) {
        ind += 1;
    }
    if has_malware_sig(data) {
        mal += 1;
        ind += 3;
    }
    if matches!(vendor_id, 0x0000 | 0xFFFF | 0x1234 | 0xDEAD | 0xBEEF) {
        ind += 2;
    }
    let level = match ind {
        0 => ThreatLevel::None,
        1..=2 => ThreatLevel::Low,
        3..=4 => ThreatLevel::Medium,
        5..=7 => ThreatLevel::High,
        _ => ThreatLevel::Critical,
    };
    ThreatAnalysis {
        level,
        indicators: ind,
        confidence: core::cmp::min(ind * 15, 100) as u8,
        malware_signatures: mal,
    }
}

pub fn analyze_firmware_behavior(trace: &[u8]) -> ThreatLevel {
    let sc = trace.windows(2).filter(|w| *w == b"SC").count() as u32;
    let net = trace.windows(3).filter(|w| *w == b"NET").count() as u32;
    let file = trace.windows(4).filter(|w| *w == b"FILE").count() as u32;
    let risk = sc / 10 + net * 5 + file * 3;
    match risk {
        0..=5 => ThreatLevel::Low,
        6..=15 => ThreatLevel::Medium,
        16..=30 => ThreatLevel::High,
        _ => ThreatLevel::Critical,
    }
}

fn has_suspicious(d: &[u8]) -> bool {
    d.windows(4).any(|w| matches!(w, b"exec" | b"fork" | b"kill" | b"root"))
}
fn high_entropy(d: &[u8]) -> bool {
    if d.is_empty() {
        return false;
    }
    let mut c = [0u32; 256];
    for &b in d {
        c[b as usize] += 1;
    }
    c.iter().filter(|&&x| x > 0).count() > 200 && d.len() > 1024
}
fn has_malware_sig(d: &[u8]) -> bool {
    const SIGS: &[&[u8]] = &[
        b"\x4d\x5a\x90\x00\x03\x00\x00\x00",
        b"\x7f\x45\x4c\x46\x02\x01\x01\x00",
        b"backdoor",
        b"rootkit",
    ];
    SIGS.iter().any(|s| d.windows(s.len()).any(|w| w == *s))
}
