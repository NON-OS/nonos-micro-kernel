use proptest::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

const CAP_FS: u64 = 1 << 0;
const CAP_IPC: u64 = 1 << 1;
const CAP_NET: u64 = 1 << 2;
const CAP_HW: u64 = 1 << 3;
const CAP_DMA: u64 = 1 << 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ProcId {
    pid: u32,
    generation: u32,
}

#[derive(Clone, Debug)]
struct ProcState {
    manifest_caps: u64,
    publisher_caps: u64,
    runtime_caps: u64,
    revocation_epoch: u64,
    alive: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CapToken {
    owner: ProcId,
    bits: u64,
    epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Endpoint {
    owner: ProcId,
    service: u16,
    epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Request {
    from: ProcId,
    to: Endpoint,
    request_id: u32,
}

#[derive(Default)]
struct Model {
    next_generation: BTreeMap<u32, u32>,
    procs: BTreeMap<ProcId, ProcState>,
    tokens: BTreeSet<CapToken>,
    endpoints: BTreeSet<Endpoint>,
    pending: BTreeSet<Request>,
}

impl Model {
    fn spawn(&mut self, pid: u32, manifest_caps: u64, publisher_caps: u64) -> ProcId {
        let generation = self.next_generation.entry(pid).and_modify(|g| *g += 1).or_insert(1);
        let id = ProcId { pid, generation: *generation };
        let runtime_caps = manifest_caps & publisher_caps;
        self.procs.insert(
            id,
            ProcState {
                manifest_caps,
                publisher_caps,
                runtime_caps,
                revocation_epoch: 0,
                alive: true,
            },
        );
        self.tokens.insert(CapToken { owner: id, bits: runtime_caps, epoch: 0 });
        id
    }

    fn register_endpoint(&mut self, id: ProcId, service: u16) {
        if let Some(proc_) = self.procs.get(&id).filter(|p| p.alive) {
            self.endpoints.insert(Endpoint { owner: id, service, epoch: proc_.revocation_epoch });
        }
    }

    fn broker_grant(&mut self, id: ProcId, requested: u64) {
        if let Some(proc_) = self.procs.get_mut(&id).filter(|p| p.alive) {
            let allowed = proc_.manifest_caps & proc_.publisher_caps;
            proc_.runtime_caps |= requested & allowed;
            self.tokens.insert(CapToken {
                owner: id,
                bits: proc_.runtime_caps,
                epoch: proc_.revocation_epoch,
            });
        }
    }

    fn revoke(&mut self, id: ProcId, revoked: u64) {
        if let Some(proc_) = self.procs.get_mut(&id) {
            proc_.runtime_caps &= !revoked;
            proc_.revocation_epoch = proc_.revocation_epoch.saturating_add(1);
            self.tokens.retain(|t| t.owner != id);
            if proc_.alive {
                self.tokens.insert(CapToken {
                    owner: id,
                    bits: proc_.runtime_caps,
                    epoch: proc_.revocation_epoch,
                });
            }
        } else {
            self.tokens.retain(|t| t.owner != id);
        }
        self.endpoints.retain(|e| e.owner != id);
        self.pending.retain(|r| r.from != id && r.to.owner != id);
    }

    fn kill(&mut self, id: ProcId) {
        if let Some(proc_) = self.procs.get_mut(&id) {
            proc_.alive = false;
            proc_.runtime_caps = 0;
            proc_.revocation_epoch = proc_.revocation_epoch.saturating_add(1);
        }
        self.tokens.retain(|t| t.owner != id);
        self.endpoints.retain(|e| e.owner != id);
        self.pending.retain(|r| r.from != id && r.to.owner != id);
    }

    fn send(&mut self, from: ProcId, to: Endpoint, request_id: u32) -> bool {
        let sender_ok = self
            .procs
            .get(&from)
            .map(|p| p.alive && (p.runtime_caps & CAP_IPC) != 0)
            .unwrap_or(false);
        let endpoint_ok = self.endpoints.contains(&to)
            && self.procs.get(&to.owner).map(|p| p.alive).unwrap_or(false);
        if sender_ok && endpoint_ok {
            self.pending.insert(Request { from, to, request_id })
        } else {
            false
        }
    }

    fn reply(&mut self, from: ProcId, original: Request) -> bool {
        if original.to.owner != from || !self.pending.remove(&original) {
            return false;
        }
        self.procs.get(&from).map(|p| p.alive).unwrap_or(false)
            && self.procs.get(&original.from).map(|p| p.alive).unwrap_or(false)
    }

    fn accept_token(&self, token: CapToken) -> bool {
        self.tokens.contains(&token)
            && self.procs.get(&token.owner).map_or(false, |p| {
                p.alive
                    && token.epoch == p.revocation_epoch
                    && (token.bits & !(p.manifest_caps & p.publisher_caps)) == 0
                    && (token.bits & !p.runtime_caps) == 0
            })
    }

    fn assert_monotonic(&self) {
        for (id, proc_) in &self.procs {
            let allowed = proc_.manifest_caps & proc_.publisher_caps;
            assert_eq!(
                proc_.runtime_caps & !allowed,
                0,
                "process {:?} gained caps outside manifest/publisher intersection",
                id
            );
            if !proc_.alive {
                assert_eq!(proc_.runtime_caps, 0, "dead process retained runtime caps");
            }
        }
        for token in &self.tokens {
            assert!(self.accept_token(*token), "stale or amplified token accepted: {:?}", token);
        }
        for endpoint in &self.endpoints {
            assert!(
                self.procs.get(&endpoint.owner).map(|p| p.alive).unwrap_or(false),
                "endpoint survived owner death: {:?}",
                endpoint
            );
        }
        for req in &self.pending {
            assert!(
                self.procs.get(&req.from).map(|p| p.alive).unwrap_or(false),
                "pending request survived client death: {:?}",
                req
            );
            assert!(
                self.endpoints.contains(&req.to),
                "pending request targets stale endpoint: {:?}",
                req
            );
        }
    }
}

fn cap_mask(raw: u8) -> u64 {
    let all = CAP_FS | CAP_IPC | CAP_NET | CAP_HW | CAP_DMA;
    (raw as u64) & all
}

proptest! {
    #[test]
    fn runtime_caps_remain_subset_across_lifecycle(trace in proptest::collection::vec(any::<u8>(), 1..256)) {
        let mut m = Model::default();
        let mut ids = [
            m.spawn(10, CAP_FS | CAP_IPC | CAP_HW, CAP_FS | CAP_IPC),
            m.spawn(11, CAP_IPC | CAP_NET | CAP_DMA, CAP_IPC | CAP_NET),
            m.spawn(12, CAP_HW | CAP_DMA, CAP_HW),
        ];

        for (step, raw) in trace.into_iter().enumerate() {
            let slot = (raw as usize) % ids.len();
            let id = ids[slot];
            match raw % 8 {
                0 => m.broker_grant(id, cap_mask(raw.rotate_left(1))),
                1 => m.revoke(id, cap_mask(raw.rotate_left(2))),
                2 => m.register_endpoint(id, raw as u16),
                3 => {
                    let target = Endpoint { owner: ids[(slot + 1) % ids.len()], service: raw as u16, epoch: 0 };
                    let _ = m.send(id, target, step as u32);
                }
                4 => m.kill(id),
                5 => ids[slot] = m.spawn(id.pid, cap_mask(raw), cap_mask(raw.rotate_left(3))),
                6 => {
                    let token = CapToken { owner: id, bits: cap_mask(raw.rotate_left(4)), epoch: 0 };
                    let _ = m.accept_token(token);
                }
                _ => {}
            }
            m.assert_monotonic();
        }
    }
}

#[test]
fn stale_reply_is_not_delivered_to_respawned_generation() {
    let mut m = Model::default();
    let client = m.spawn(1, CAP_IPC, CAP_IPC);
    let server_n = m.spawn(2, CAP_IPC, CAP_IPC);
    m.register_endpoint(server_n, 7);
    let endpoint_n = Endpoint { owner: server_n, service: 7, epoch: 0 };
    assert!(m.send(client, endpoint_n, 42));
    let stale = Request { from: client, to: endpoint_n, request_id: 42 };

    m.kill(server_n);
    let server_next = m.spawn(2, CAP_IPC, CAP_IPC);
    m.register_endpoint(server_next, 7);

    assert!(!m.reply(server_next, stale), "stale reply crossed process generation");
    m.assert_monotonic();
}

#[test]
fn pid_reuse_does_not_resurrect_capability_token() {
    let mut m = Model::default();
    let first = m.spawn(8, CAP_HW | CAP_DMA, CAP_HW | CAP_DMA);
    let old = CapToken { owner: first, bits: CAP_HW | CAP_DMA, epoch: 0 };
    assert!(m.accept_token(old));

    m.kill(first);
    let second = m.spawn(8, CAP_HW, CAP_HW);

    assert_ne!(first, second);
    assert!(!m.accept_token(old), "old generation token accepted after pid reuse");
    m.assert_monotonic();
}

#[test]
fn hardware_grants_are_constrained_by_manifest_and_publisher_caps() {
    let mut m = Model::default();
    let driver = m.spawn(3, CAP_HW, CAP_HW | CAP_DMA);
    m.broker_grant(driver, CAP_HW | CAP_DMA);

    let proc_ = m.procs.get(&driver).expect("driver exists");
    assert_eq!(proc_.runtime_caps & CAP_HW, CAP_HW);
    assert_eq!(proc_.runtime_caps & CAP_DMA, 0, "DMA grant escaped manifest ceiling");
    m.assert_monotonic();
}
