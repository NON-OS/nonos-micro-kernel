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

// Service support kept in the kernel.
//
// This module is limited to capability bits, capsule liveness state,
// and the endpoint registry used by kernel-side IPC clients. Service
// implementations run as userland capsules; the old in-kernel client,
// server, protocol, and *_engine framework has been removed.

pub mod caps;
pub mod lifecycle;
pub mod registry;

pub use caps::{check_service_cap, has_capability, verify_caller_cap, CapError, ServiceCap};
pub use caps::{
    CAP_ADMIN, CAP_APPS, CAP_CRYPTO, CAP_DISPLAY, CAP_DRIVER, CAP_INPUT, CAP_NET, CAP_VFS,
};
pub use registry::{
    lookup_port, lookup_service, register_endpoint, required_caps, RegError, ServiceEndpoint,
};
