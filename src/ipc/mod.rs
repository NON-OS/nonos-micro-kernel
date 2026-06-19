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

// IPC primitives.
//
//   * `nonos_inbox`   — bounded per-name queues; capsules enqueue/dequeue here.
//   * `nonos_channel` — wire envelope (`IpcMessage`) + MAC-key bootstrap.
//   * `kernel_ipc`    — router helpers consumed by `syscall::microkernel::ipc`.
//   * `pipe`          — FIFO used by `process::fd_table`.

pub mod kernel_ipc;
pub mod nonos_channel;
pub mod nonos_inbox;
pub mod pipe;

pub use nonos_channel as channel;
