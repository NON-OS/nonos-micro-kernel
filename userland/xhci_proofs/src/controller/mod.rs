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

//! The controller files that talk to registers and nothing else.
//!
//! Each is the shipping file. `park` is public here because `reset_port`
//! reaches it as `crate::controller::park`, the same path it has in the
//! capsule. The handoff's two files are included as siblings so `claim`
//! resolves through `super` exactly as it does in the capsule.

#[path = "../../../capsule_driver_xhci/src/controller/legacy_handoff/claim.rs"]
mod claim;
#[path = "../../../capsule_driver_xhci/src/controller/halt.rs"]
mod halt;
#[path = "../../../capsule_driver_xhci/src/controller/legacy_handoff/legacy_handoff.rs"]
mod legacy_handoff;
#[path = "../../../capsule_driver_xhci/src/controller/park.rs"]
pub mod park;
#[path = "../../../capsule_driver_xhci/src/controller/refuse_unsupported.rs"]
mod refuse_unsupported;
#[path = "../../../capsule_driver_xhci/src/controller/reset.rs"]
mod reset;
#[path = "../../../capsule_driver_xhci/src/controller/reset_port.rs"]
mod reset_port;
#[path = "../../../capsule_driver_xhci/src/controller/start.rs"]
mod start;
#[path = "../../../capsule_driver_xhci/src/controller/wait_cnr_clear.rs"]
mod wait_cnr_clear;
#[path = "../../../capsule_driver_xhci/src/controller/wait_hc_running.rs"]
mod wait_hc_running;

pub use halt::halt;
pub use legacy_handoff::legacy_handoff;
pub use refuse_unsupported::refuse_unsupported;
pub use reset::reset;
pub use reset_port::reset_port;
pub use start::start;
pub use wait_cnr_clear::wait_cnr_clear;
pub use wait_hc_running::wait_hc_running;
