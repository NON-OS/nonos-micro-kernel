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

use nonos_cap::{
    CAP_CRYPTO, CAP_DEBUG, CAP_ENROL_DEV_ROOT, CAP_FILESYSTEM, CAP_GRAPHICS_DISPLAY_QUERY,
    CAP_GRAPHICS_PRESENT, CAP_GRAPHICS_SURFACE_CREATE, CAP_GRAPHICS_SURFACE_MAP, CAP_IPC,
    CAP_NETWORK, CAP_REGISTER_SERVICE,
};

// Groups are stated as intent, not as bits. A developer knows they want a
// window; they should not have to know that a window is four separate
// permissions, or which four. The kernel still enforces the bits, so the
// grouping costs nothing in precision and buys a declaration a reviewer can
// read at a glance.

/// Draw to the screen and own a window.
pub const WINDOW: u64 = CAP_GRAPHICS_DISPLAY_QUERY
    | CAP_GRAPHICS_SURFACE_CREATE
    | CAP_GRAPHICS_SURFACE_MAP
    | CAP_GRAPHICS_PRESENT;

/// Reach the network. Whether that traffic is forced through the mixnet is a
/// separate matter the system decides, not the app.
pub const NETWORK: u64 = CAP_NETWORK;

/// Read and write files.
pub const STORAGE: u64 = CAP_FILESYSTEM;

/// Use the kernel's cryptographic services, including the random source.
pub const CRYPTO: u64 = CAP_CRYPTO;

/// Talk to other capsules by message.
pub const IPC: u64 = CAP_IPC;

/// Answer on a named endpoint, so other capsules can find this one. Implies
/// IPC, because a service nobody can message is not a service.
pub const SERVICE: u64 = CAP_REGISTER_SERVICE | CAP_IPC;

/// Write to the boot log. Useful while developing, and worth removing before
/// publishing: a shipped app that narrates itself to the console is leaking
/// whatever it narrates.
pub const DEBUG: u64 = CAP_DEBUG;

/// Ask the machine to trust a signing root, so software built here can run
/// here. Held by a build capsule and nothing else.
///
/// Declaring it does not grant the power to approve: enrolment needs a code
/// the kernel prints to a console no capsule can read. This bit only buys the
/// right to ask.
pub const BUILD_TOOLING: u64 = CAP_ENROL_DEV_ROOT;
