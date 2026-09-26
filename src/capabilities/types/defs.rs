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

//! Every capability the kernel knows, with the bit it occupies.

use super::table::capability_table;

capability_table! {
    CoreExec = 1 << 0,
    /// Enforces nothing.
    IO = 1 << 1,
    Network = 1 << 2,
    IPC = 1 << 3,
    Memory = 1 << 4,
    Crypto = 1 << 5,
    FileSystem = 1 << 6,
    /// Enforces nothing.
    Hardware = 1 << 7,
    Debug = 1 << 8,
    Admin = 1 << 9,
    RegisterService = 1 << 10,
    GraphicsDisplayQuery = 1 << 11,
    GraphicsSurfaceCreate = 1 << 12,
    GraphicsSurfaceMap = 1 << 13,
    GraphicsPresent = 1 << 14,
    /// Enumerate-only. Claiming a device is `Driver`.
    DeviceEnum = 1 << 15,
    /// Claim and release a device through the broker.
    Driver = 1 << 16,
    /// Map a slice of a claimed device's BAR into the holder's own
    /// address space.
    Mmio = 1 << 17,
    /// Bind a claimed device's interrupt to a kernel-delivered
    /// notification slot.
    Irq = 1 << 18,
    /// Receive a DMA-coherent buffer a claimed device may read or
    /// write through.
    Dma = 1 << 19,
    /// Mint a PIO grant against a port BAR and run kernel-mediated
    /// `in` and `out` on its ports.
    Pio = 1 << 20,
    InputSource = 1 << 21,
    TimeSet = 1 << 22,
    /// Attribute a capsule load's parent to a kernel-attested pid other than
    /// the caller.
    SpawnBroker = 1 << 23,
    /// Spawn another instance of an already embedded, attested app capsule.
    SpawnWindow = 1 << 24,
    /// Terminate a process the caller does not parent.
    ProcessControl = 1 << 25,
    StoreWrite = 1 << 26,
    /// Enrol a signing root, so capsules built on this machine run on it.
    EnrolDevRoot = 1 << 27,
    /// Reach the keyring capsule, which stores and derives key material.
    Keyring = 1 << 28,
    /// Draw from the entropy capsule.
    Entropy = 1 << 29,
    /// Reach the marketplace and install a capsule. Held by the
    /// installer, not by every capsule that draws a window.
    AppInstall = 1 << 30,
    /// Read the attestation registry: which capsules are running,
    /// their measurements and their capability masks.
    AttestRead = 1 << 31,
    /// Host code this kernel has not verified: create a process with no
    /// capabilities, build its address space, and answer the syscalls it makes
    /// that the kernel refuses.
    ForeignExec = 1 << 32,
    /// Mint a proof that this machine agreed to run bytes it installed itself.
    LocalSign = 1 << 33,
}
