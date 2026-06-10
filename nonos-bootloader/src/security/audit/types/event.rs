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

pub const AUDIT_MSG_LEN: usize = 48;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AuditEvent {
    BootStart = 0x01,
    UefiInit = 0x02,
    SecureBootCheck = 0x03,
    TpmInit = 0x04,
    EntropyCollect = 0x05,
    KeysLoaded = 0x06,
    KernelLoaded = 0x07,
    HashComputed = 0x08,
    SignatureVerified = 0x09,
    SignatureFailed = 0x0A,
    ZkProofVerified = 0x0B,
    ZkProofFailed = 0x0C,
    PolicyEnforced = 0x0D,
    PolicyViolation = 0x0E,
    ExitBootServices = 0x0F,
    KernelHandoff = 0x10,
    SecurityAlert = 0xFF,
}
