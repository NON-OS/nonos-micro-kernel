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

mod api;
mod csprng;
mod entropy;
mod error;
pub mod global;

pub use api::{fill_bytes, secure_random_u64};

pub use csprng::{ChaChaRng, RESEED_INTERVAL};

pub use entropy::{
    collect_seed_entropy, collect_seed_entropy_secure, cpu_entropy64, cpu_random64, get_entropy64,
    get_entropy64_secure, get_tsc_entropy, has_adequate_entropy, has_cpu_entropy, has_cpu_random,
    init_entropy, mark_bootloader_entropy_provided, mix_entropy_into_seed, verify_entropy_sources,
    EntropyError,
};

pub use error::{RngError, RngResult};

pub use global::{
    fill_random_bytes, fill_random_bytes_secure, get_random_bytes, get_random_bytes_secure,
    init_rng, init_rng_simple, is_initialized, random_range, random_range_secure, random_u32,
    random_u32_secure, random_u64, random_u64_secure, seed_direct, seed_from_bootloader, seed_rng,
    GLOBAL_COUNTER,
};
