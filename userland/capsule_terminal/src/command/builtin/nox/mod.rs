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
mod battery;
mod caps;
mod children;
mod clear;
mod copy;
mod date;
mod dispatch;
mod display;
mod du;
mod echo;
mod ensure_pid;
mod enter;
mod exec;
mod find;
mod help;
mod history;
mod http;
mod id;
mod ifconfig;
pub mod install;
mod keep;
mod kill;
mod ls;
mod mk;
mod motd;
mod mv;
mod nslookup;
mod nym;
mod pathname;
mod ping;
mod pkg;
mod pull;
mod push;
mod read;
mod rm;
mod run;
mod set;
mod stat;
mod svc;
mod sysinfo;
mod touch;
mod unalias;
mod unknown;
mod unset;
mod uptime;
mod whereis;
mod write;

pub use dispatch::dispatch;
