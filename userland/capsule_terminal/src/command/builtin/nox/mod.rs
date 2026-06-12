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

mod alias;
mod apps;
mod caps;
mod children;
mod clear;
mod copy;
mod display;
mod dispatch;
mod echo;
mod ensure_pid;
mod enter;
mod help;
mod history;
mod id;
mod ls;
mod mk;
mod motd;
mod mv;
mod ping;
mod read;
mod rm;
mod run;
mod set;
mod stat;
mod svc;
mod sysinfo;
mod unalias;
mod unknown;
mod unset;
mod whereis;
mod write;

pub use dispatch::dispatch;
