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

mod collect;
mod error;
mod hardware;
mod state;

pub use collect::{
    collect_seed_entropy, collect_seed_entropy_secure, get_entropy64, get_entropy64_secure,
    get_tsc_entropy, has_adequate_entropy, init_entropy, mark_bootloader_entropy_provided,
    mix_entropy_into_seed, verify_entropy_sources,
};
pub use error::EntropyError;
pub use hardware::{cpu_entropy64, cpu_random64, has_cpu_entropy, has_cpu_random};
