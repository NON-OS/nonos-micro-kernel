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

extern crate alloc;

use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use spin::RwLock;

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub enable_cet: bool,
    pub enable_smep: bool,
    pub enable_smap: bool,
    pub enable_umip: bool,
    pub enable_mpx: bool,
    pub enable_cfi: bool,
    pub enable_stack_canaries: bool,
    pub enable_aslr: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_cet: true,
            enable_smep: true,
            enable_smap: true,
            enable_umip: true,
            enable_mpx: true,
            enable_cfi: true,
            enable_stack_canaries: true,
            enable_aslr: true,
        }
    }
}

pub struct StackCanary {
    global: AtomicU64,
    per_cpu: RwLock<BTreeMap<u32, u64>>,
    rotations: AtomicU64,
}

impl StackCanary {
    pub fn new() -> Self {
        Self {
            global: AtomicU64::new(Self::gen_canary()),
            per_cpu: RwLock::new(BTreeMap::new()),
            rotations: AtomicU64::new(0),
        }
    }
    fn gen_canary() -> u64 {
        if let Some(v) = crate::arch::cpu_random::random_u64() {
            return v;
        }
        // No hardware generator on this CPU. A canary derived from the cycle
        // counter is weaker than a drawn one, but it still differs per boot and
        // per rotation, which is what a canary needs.
        crate::arch::read_time_counter() ^ 0xDEAD_BEEF_CAFE_BABE
    }
    pub fn get(&self, cpu: u32) -> u64 {
        let lock = self.per_cpu.read();
        if let Some(c) = lock.get(&cpu) {
            *c
        } else {
            self.global.load(Ordering::Relaxed)
        }
    }
    pub fn rotate(&self) {
        let new = Self::gen_canary();
        self.global.store(new, Ordering::Release);
        let mut lock = self.per_cpu.write();
        for (_, c) in lock.iter_mut() {
            *c = Self::gen_canary() ^ new;
        }
        self.rotations.fetch_add(1, Ordering::Release);
    }
    pub fn verify(&self, cpu: u32, value: u64) -> bool {
        value == self.get(cpu)
    }
}

pub struct CFI {
    enabled: AtomicBool,
    targets: RwLock<BTreeMap<u64, u32>>,
    violations: AtomicU64,
}
impl CFI {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            targets: RwLock::new(BTreeMap::new()),
            violations: AtomicU64::new(0),
        }
    }
    pub fn add_target(&self, addr: u64, sig: u32) {
        self.targets.write().insert(addr, sig);
    }
    pub fn validate(&self, addr: u64) -> bool {
        if !self.enabled.load(Ordering::Relaxed) {
            return true;
        }
        let lock = self.targets.read();
        lock.contains_key(&addr)
    }
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
    }
}

pub struct ASLR {
    entropy_bits: u32,
    enabled: AtomicBool,
}
impl ASLR {
    pub fn new(entropy: u32) -> Self {
        Self { entropy_bits: entropy, enabled: AtomicBool::new(true) }
    }
    pub fn randomize(&self, base: u64) -> u64 {
        if !self.enabled.load(Ordering::Relaxed) {
            return base;
        }
        let slide = crate::crypto::secure_random_u64() & ((1 << self.entropy_bits) - 1);
        (base + (slide << 12)) & 0xFFFF_FFFF_FFFF_F000
    }
}

pub struct AdvancedSecurityManager {
    pub config: SecurityConfig,
    pub stack_canary: StackCanary,
    pub cfi: CFI,
    pub aslr: ASLR,
    pub violations: AtomicU64,
}

impl AdvancedSecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            stack_canary: StackCanary::new(),
            cfi: CFI::new(),
            aslr: ASLR::new(28),
            config,
            violations: AtomicU64::new(0),
        }
    }
    pub fn init(&self) {
        if self.config.enable_stack_canaries {
            self.stack_canary.rotate();
        }
        if self.config.enable_cfi {
            self.cfi.enable();
        }
        if self.config.enable_aslr {
            self.aslr.enabled.store(true, Ordering::Release);
        }
    }
    pub fn report_violation(&self, msg: &str) {
        self.violations.fetch_add(1, Ordering::Release);
        crate::log::security_log!("SECURITY VIOLATION: {}", msg);
    }
    pub fn stats(&self) -> SecurityStats {
        SecurityStats {
            violations: self.violations.load(Ordering::Relaxed),
            canary_rotations: self.stack_canary.rotations.load(Ordering::Relaxed),
            cfi_violations: self.cfi.violations.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug)]
pub struct SecurityStats {
    pub violations: u64,
    pub canary_rotations: u64,
    pub cfi_violations: u64,
}

static GLOBAL_MANAGER: spin::Once<AdvancedSecurityManager> = spin::Once::new();

pub fn init_advanced_security() -> Result<(), &'static str> {
    let manager = AdvancedSecurityManager::new(SecurityConfig::default());
    manager.init();
    GLOBAL_MANAGER.call_once(|| manager);
    Ok(())
}

pub fn security_manager() -> &'static AdvancedSecurityManager {
    GLOBAL_MANAGER.call_once(|| {
        let manager = AdvancedSecurityManager::new(SecurityConfig::default());
        manager.init();
        manager
    })
}

static ENFORCE_WX: AtomicBool = AtomicBool::new(true);
static ENFORCE_NX_STACK: AtomicBool = AtomicBool::new(true);

pub fn enforce_wx_policy() -> bool {
    ENFORCE_WX.load(Ordering::Relaxed)
}
pub fn enforce_nx_stack() -> bool {
    ENFORCE_NX_STACK.load(Ordering::Relaxed)
}
pub fn set_wx_policy(enforce: bool) {
    ENFORCE_WX.store(enforce, Ordering::Relaxed);
}
pub fn set_nx_stack_policy(enforce: bool) {
    ENFORCE_NX_STACK.store(enforce, Ordering::Relaxed);
}
