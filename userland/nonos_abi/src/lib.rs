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

#![no_std]

mod call;
mod diverging;
mod input;
mod mmap;
mod munmap;
mod numbers;
mod raw;

pub use call::syscall;
pub use diverging::syscall_diverging;
pub use input::{
    input_drain, InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_KEY_DOWN,
    INPUT_KIND_KEY_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH,
    INPUT_KIND_WHEEL,
};
pub use mmap::mmap;
pub use munmap::munmap;
pub use numbers::{
    N_CRYPTO_RANDOM, N_DEBUG, N_DISPLAY_DIMENSIONS, N_EXIT, N_INPUT_EVENT_DRAIN, N_IPC_CALL,
    N_IPC_RECV, N_IPC_RECV_FROM, N_IPC_REPLY, N_IPC_SEND, N_IPC_SEND_TO_PID, N_MMAP, N_MUNMAP,
    N_SERVICE_LOOKUP, N_SERVICE_REGISTER, N_SURFACE_ATTACH, N_SURFACE_PRESENT, N_SURFACE_REGISTER,
    N_SURFACE_RELEASE, N_SURFACE_SHARE, N_TIME_MILLIS, N_YIELD,
};
