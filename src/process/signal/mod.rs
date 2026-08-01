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

pub mod constants;
// Delivering a signal means writing the register frame the CPU returns into, and
// holding the frame it interrupted. Both are the x86_64 register file today; the
// aarch64 equivalent goes in beside the exception frame in `arch::aarch64`.
#[cfg(target_arch = "x86_64")]
pub mod delivery;
pub mod error;
#[cfg(target_arch = "x86_64")]
pub mod frame;
pub mod helpers;
pub mod name;
pub mod queued;
pub mod send;
pub mod set;
pub mod sigaction;
pub mod siginfo;
pub mod state;

pub use constants::*;
pub use error::SignalError;
pub use helpers::{
    can_be_blocked, can_be_caught, can_be_ignored, generates_core_dump, is_fatal_by_default,
    is_rt_signal, is_stop_signal, is_synchronous, is_valid_signal,
};
pub use name::{signal_from_name, signal_name};
pub use queued::QueuedSignal;
pub use send::{
    clear_pending_signal, get_pending_signal, has_pending_signals, send_signal, send_signal_info,
    send_signal_to_group,
};
pub use set::{SignalSet, SignalSetIter};
pub use sigaction::{KernelSigaction, Sigaction, SigactionFlags};
pub use siginfo::{KernelSigInfo, SigCode, SigInfo};
pub use state::SignalState;
