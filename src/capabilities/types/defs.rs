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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    CoreExec,
    IO,
    Network,
    IPC,
    Memory,
    Crypto,
    FileSystem,
    Hardware,
    Debug,
    Admin,
    RegisterService,
    GraphicsDisplayQuery,
    GraphicsSurfaceCreate,
    GraphicsSurfaceMap,
    GraphicsPresent,
    DeviceEnum,
    // Driver-broker authority. `DeviceEnum` is enumerate-only;
    // `Driver` lets a capsule claim and release a device; `Mmio`
    // lets a claim holder map a slice of a BAR into its own AS;
    // `Irq` lets a claim holder bind a device interrupt to a
    // kernel-delivered notification slot; `Dma` lets a claim
    // holder receive a DMA-coherent buffer the device is allowed
    // to read or write through; `Pio` lets a claim holder mint a
    // PIO grant against a port BAR and execute kernel-mediated
    // `in`/`out` instructions on its ports.
    Driver,
    Mmio,
    Irq,
    Dma,
    Pio,
    InputSource,
    TimeSet,
    // Grants a spawn site the authority to attribute a capsule-load's
    // parent to a kernel-attested pid other than the caller itself
    // (see `on_behalf_of` on `CapsuleLoadRequest`). No ordinary capsule
    // manifest declares this bit; only the installer's does.
    SpawnBroker,
    // Authority to ask the kernel to spawn another instance of an already
    // embedded, attested app capsule (a fresh window). Held only by the
    // desktop shell so a compromised app cannot spawn capsules.
    SpawnWindow,
    // Authority to terminate a process the caller does not parent. Every
    // capsule may already kill its own children; this bit extends that to
    // arbitrary pids, held only by the process manager so a compromised app
    // cannot kill unrelated capsules.
    ProcessControl,
    StoreWrite,
    // Authority to enrol a signing root, so capsules built on this machine can
    // run on it. Its own bit rather than folded into `Admin`: enrolling a root
    // changes what the machine will execute for the rest of the session, and
    // nothing that merely needs administrative reach should acquire that by
    // side effect.
    EnrolDevRoot,
    // Authority to reach the keyring capsule, which stores and derives key
    // material. Not folded into `Crypto`: `Crypto` is permission to perform an
    // operation with a key the caller already holds, this is permission to
    // reach the store the keys live in. A capsule that encrypts its own data
    // needs the first and must not acquire the second with it.
    Keyring,
    // Authority to draw from the entropy capsule. Separate from `Crypto` for
    // the same reason, and separate from `Admin`, which the reseed path
    // requires: drawing randomness and replacing the source of it are not the
    // same authority.
    Entropy,
    // Authority to reach the marketplace and install a capsule. Held by the
    // installer, not by every capsule that happens to draw a window.
    AppInstall,
}
