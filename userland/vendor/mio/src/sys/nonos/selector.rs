// NONOS mio selector. NONOS has no epoll; readiness comes from the net.sockets
// service via its OP_POLL round trip (non-consuming can_recv/can_send), the
// same primitive nonos_async's reactor uses. A registered `RawFd` resolves to
// its net.sockets handle through the os::fd table seam
// (`std::os::fd::nonos_socket_handle`), and the selector polls each registered
// handle per `select` call.
//
// The single subtlety: a real `epoll_wait` blocks until readiness or the
// timeout, but one OP_POLL round is single-shot. So `select` LOOPS: poll all
// registrations, return as soon as any is ready (or the waker fired), else
// cooperatively yield and re-poll until the deadline. Returning immediately
// would spin the tokio runtime at 100%; never returning would hang it.

use std::io;
use std::os::fd::RawFd;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::net::{poll_handle, POLL_READABLE, POLL_WRITABLE};
use super::syscall::{sys0, tag4};
use crate::{Interest, Token};

// Upper bound on concurrently registered sources. Fixed so the selector needs
// no reallocation under the lock and a registration leak fails closed.
const MAX_REGS: usize = 1024;

#[derive(Clone, Copy)]
struct Reg {
    fd: RawFd,
    token: Token,
    interest: u8,
    active: bool,
}

struct Inner {
    regs: [Reg; MAX_REGS],
}

pub struct Selector {
    #[cfg(debug_assertions)]
    id: usize,
    // Shared across clones: tokio registers I/O sources through a `Registry`
    // (a clone of this selector) while the runtime parks by polling the
    // original, so both must see the same registration table. Real mio shares
    // one epoll fd across clones; here an Arc shares the reg set.
    inner: Arc<Mutex<Inner>>,
}

#[cfg(debug_assertions)]
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

// The wake path is process-global. A tokio runtime has a single selector (the
// Registry clones it, sharing the same wake), and a Waker created from that
// selector must set this from any thread without holding a borrow, mirroring
// how the epoll backend shares one eventfd. WAKER_TOKEN holds the token tokio's
// Waker registered; usize::MAX means "no waker registered".
static WOKEN: AtomicBool = AtomicBool::new(false);
static WAKER_TOKEN: AtomicUsize = AtomicUsize::new(usize::MAX);

// Raise the wake so an in-flight select() on any selector returns at once.
pub(crate) fn signal_wake() {
    WOKEN.store(true, Ordering::Release);
}

pub(crate) fn set_waker_token(token: Token) {
    WAKER_TOKEN.store(token.0, Ordering::Release);
}

impl Selector {
    pub fn new() -> io::Result<Selector> {
        let empty = Reg { fd: -1, token: Token(0), interest: 0, active: false };
        Ok(Selector {
            #[cfg(debug_assertions)]
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            inner: Arc::new(Mutex::new(Inner { regs: [empty; MAX_REGS] })),
        })
    }

    pub fn try_clone(&self) -> io::Result<Selector> {
        // The clone must observe the SAME registrations tokio makes through the
        // Registry: share the reg table (and the process-global wake path) so a
        // source registered via the Registry is seen by the poll on the original.
        Ok(Selector {
            #[cfg(debug_assertions)]
            id: self.id,
            inner: Arc::clone(&self.inner),
        })
    }

    #[cfg(debug_assertions)]
    pub fn id(&self) -> usize {
        self.id
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        match self.inner.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        }
    }

    pub fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        let mut inner = self.lock();
        for r in inner.regs.iter_mut() {
            if !r.active {
                *r = Reg { fd, token, interest: to_bits(interests), active: true };
                return Ok(());
            }
        }
        Err(io::Error::new(io::ErrorKind::Other, "mio selector registration table full"))
    }

    pub fn reregister(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        let mut inner = self.lock();
        for r in inner.regs.iter_mut() {
            if r.active && r.fd == fd {
                r.token = token;
                r.interest = to_bits(interests);
                return Ok(());
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "reregister of an unregistered fd"))
    }

    pub fn deregister(&self, fd: RawFd) -> io::Result<()> {
        let mut inner = self.lock();
        for r in inner.regs.iter_mut() {
            if r.active && r.fd == fd {
                r.active = false;
                return Ok(());
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "deregister of an unregistered fd"))
    }

    pub fn select(&self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
        events.clear();
        let deadline = timeout.map(|t| Instant::now() + t);

        loop {
            // A wake from another task takes priority and returns at once with
            // the waker's token so the runtime services timers/spawns.
            if WOKEN.swap(false, Ordering::AcqRel) {
                let tok = WAKER_TOKEN.load(Ordering::Acquire);
                if tok != usize::MAX {
                    events.push(Event { token: Token(tok), readable: true, writable: false });
                }
                return Ok(());
            }

            // One readiness sweep over every active registration.
            {
                let inner = self.lock();
                for r in inner.regs.iter() {
                    if !r.active {
                        continue;
                    }
                    let bits = match std::os::fd::nonos_socket_handle(r.fd) {
                        Some(handle) => poll_handle(handle),
                        // A registered fd that is not a live socket (e.g. a
                        // closed handle) reports no readiness; tokio will
                        // deregister it on error from its own I/O.
                        None => 0,
                    };
                    let readable = bits & POLL_READABLE != 0 && r.interest & POLL_READABLE != 0;
                    let writable = bits & POLL_WRITABLE != 0 && r.interest & POLL_WRITABLE != 0;
                    if readable || writable {
                        events.push(Event { token: r.token, readable, writable });
                    }
                }
            }

            if !events.is_empty() {
                return Ok(());
            }

            // Nothing ready. Respect a zero/elapsed deadline by returning empty
            // (a non-blocking poll); otherwise park briefly and sweep again.
            match deadline {
                Some(d) => {
                    if Instant::now() >= d {
                        return Ok(());
                    }
                    park();
                }
                None => park(),
            }
        }
    }
}

fn to_bits(interests: Interest) -> u8 {
    let mut bits = 0;
    if interests.is_readable() {
        bits |= POLL_READABLE;
    }
    if interests.is_writable() {
        bits |= POLL_WRITABLE;
    }
    bits
}

// Cooperative park between poll rounds. NONOS has no timed-sleep syscall, so
// this hands the core to other capsules with a single yield; select()'s own
// monotonic-clock deadline check bounds how long the loop keeps re-polling.
fn park() {
    // SAFETY: MYLD takes no arguments and only reschedules the caller.
    unsafe {
        sys0(tag4(b"MYLD"));
    }
}

// mio's per-event view. Unlike the epoll backend's raw `epoll_event`, NONOS has
// no C event struct, so this is a plain record the `event` accessors read.
#[derive(Debug, Clone, Copy)]
pub struct Event {
    token: Token,
    readable: bool,
    writable: bool,
}

pub type Events = Vec<Event>;

pub mod event {
    use super::Event;
    use crate::Token;
    use std::fmt;

    pub fn token(event: &Event) -> Token {
        event.token
    }

    pub fn is_readable(event: &Event) -> bool {
        event.readable
    }

    pub fn is_writable(event: &Event) -> bool {
        event.writable
    }

    pub fn is_error(_event: &Event) -> bool {
        false
    }

    pub fn is_read_closed(_event: &Event) -> bool {
        false
    }

    pub fn is_write_closed(_event: &Event) -> bool {
        false
    }

    pub fn is_priority(_event: &Event) -> bool {
        false
    }

    pub fn is_aio(_event: &Event) -> bool {
        false
    }

    pub fn is_lio(_event: &Event) -> bool {
        false
    }

    pub fn debug_details(f: &mut fmt::Formatter<'_>, event: &Event) -> fmt::Result {
        write!(
            f,
            "token: {:?}, readable: {}, writable: {}",
            event.token, event.readable, event.writable
        )
    }
}
