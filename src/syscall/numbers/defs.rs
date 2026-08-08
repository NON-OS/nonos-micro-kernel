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

// Active NØNOS syscall ABI. Discriminants are 4-byte ASCII tags
// packed little-endian via `tag4`; the registry in
// `crate::syscall::abi::REGISTRY` is the source of truth.

use crate::syscall::abi::tag4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum SyscallNumber {
    CryptoRandom = tag4(b"CRND"),
    CryptoHash = tag4(b"CHSH"),
    CryptoEncrypt = tag4(b"CENC"),
    CryptoDecrypt = tag4(b"CDEC"),
    CryptoEncryptAad = tag4(b"CEAD"),
    CryptoDecryptAad = tag4(b"CDAD"),
    CryptoEd25519Verify = tag4(b"CEDV"),
    CryptoEd25519Sign = tag4(b"CEDS"),
    CryptoEd25519Pubkey = tag4(b"CEDP"),
    CryptoX25519Public = tag4(b"CXPK"),
    CryptoX25519Shared = tag4(b"CXSH"),
    CryptoHmacSha256 = tag4(b"CHMC"),
    CryptoHkdfSha256 = tag4(b"CHKF"),
    CryptoKeccak256 = tag4(b"CKEC"),
    CryptoSecp256k1Sign = tag4(b"CSKS"),
    CryptoSecp256k1Pubkey = tag4(b"CSPB"),
    AdminReboot = tag4(b"ARBT"),
    AdminShutdown = tag4(b"ASDN"),
    AdminPolicyPush = tag4(b"APPS"),
    GraphicsDisplayDimensions = tag4(b"GDIM"),
    MkIpcSend = tag4(b"MISD"),
    MkIpcRecv = tag4(b"MIRC"),
    MkIpcCall = tag4(b"MICL"),
    MkIpcRecvFrom = tag4(b"MIRF"),
    MkIpcReply = tag4(b"MIRY"),
    MkIpcSendToPid = tag4(b"MISP"),
    MkServiceLookup = tag4(b"MSVL"),
    MkServiceRegister = tag4(b"MSVR"),
    MkMmap = tag4(b"MMAP"),
    MkMunmap = tag4(b"MUMP"),
    MkSpawn = tag4(b"MSPN"),
    MkCapsuleLoad = tag4(b"MCLD"),
    MkCapsuleVerify = tag4(b"MCVF"),
    MkExit = tag4(b"MEXT"),
    MkPidAlive = tag4(b"MPAL"),
    MkWait = tag4(b"MWAT"),
    MkKill = tag4(b"MKIL"),
    MkGetPid = tag4(b"MGPD"),
    MkArgs = tag4(b"MKAR"),
    MkThreadSpawn = tag4(b"MTSP"),
    MkSetTls = tag4(b"MSTB"),
    MkYield = tag4(b"MYLD"),
    MkFutexWait = tag4(b"MFTW"),
    MkFutexWake = tag4(b"MFTK"),
    MkTimeMillis = tag4(b"MTMS"),
    MkTimeMonotonic = tag4(b"MMON"),
    MkTimeRtc = tag4(b"MTRT"),
    MkTimeAdjust = tag4(b"MTAD"),
    MkBatteryStatus = tag4(b"MBAT"),
    MkProcStat = tag4(b"MPST"),
    MkProcOutput = tag4(b"MOUT"),
    MkProcInput = tag4(b"MPIN"),
    MkStdinRead = tag4(b"MSRD"),
    MkStdoutWrite = tag4(b"MSOW"),
    MkStoreWrite = tag4(b"MSWR"),
    MkAttestStatus = tag4(b"MAST"),
    MkToolRun = tag4(b"MTRN"),
    MkCapGrant = tag4(b"MCGT"),
    MkCapRevoke = tag4(b"MCRV"),
    MkCapCheck = tag4(b"MCCK"),
    MkDeviceList = tag4(b"MDLS"),
    MkDeviceClaim = tag4(b"MDCL"),
    MkDeviceRelease = tag4(b"MDRL"),
    MkMmioMap = tag4(b"MMMP"),
    MkMmioUnmap = tag4(b"MMUM"),
    MkIrqBind = tag4(b"MIRB"),
    MkIrqUnbind = tag4(b"MIRU"),
    MkIrqAck = tag4(b"MIRA"),
    MkIrqPoll = tag4(b"MIRP"),
    MkIrqWait = tag4(b"MIRW"),
    MkDmaMap = tag4(b"MDMM"),
    MkDmaUnmap = tag4(b"MDMU"),
    MkPciConfigRead = tag4(b"MPCR"),
    MkPciConfigWrite = tag4(b"MPCW"),
    MkPioGrant = tag4(b"MPGT"),
    MkPioRead = tag4(b"MPRD"),
    MkPioWrite = tag4(b"MPWR"),
    MkPioRelease = tag4(b"MPRL"),
    MkDebug = tag4(b"MDBG"),
    MkSurfaceRegister = tag4(b"MSRG"),
    MkSurfaceShare = tag4(b"MSSH"),
    MkSurfaceAttach = tag4(b"MSAT"),
    MkSurfaceRelease = tag4(b"MSRL"),
    MkSurfacePresent = tag4(b"MSPR"),
    MkDisplayVsyncWait = tag4(b"MDVW"),
    MkInputEventPost = tag4(b"MIEP"),
    MkInputEventDrain = tag4(b"MIED"),
    MkInputEventWait = tag4(b"MIEW"),
    MkSpawnInstance = tag4(b"MSPI"),
}
