// NONOS mio backend. NONOS has no epoll and no kernel file descriptors; I/O is
// IPC to the net.sockets service, and readiness is its non-consuming OP_POLL.
// The selector adapts that single-shot poll into mio's blocking `select`, the
// waker breaks an in-flight select from another task, and the net modules
// bridge socket creation and connect/bind/listen/accept onto net.sockets,
// producing os::fd-table-backed descriptors whose std read/write already flow
// through the PAL. See MIO-NONOS-DESIGN.md.

mod net;
mod selector;
mod syscall;

pub(crate) use self::selector::{event, Event, Events, Selector};

mod waker;
pub(crate) use self::waker::Waker;

cfg_net! {
    pub(crate) mod tcp;
    pub(crate) mod udp;
}

cfg_io_source! {
    use std::io;
    use std::os::fd::RawFd;

    use crate::{Interest, Registry, Token};

    // NONOS holds no per-source reactor state beyond the selector's
    // registration table, so IoSourceState is a marker, and do_io just runs
    // the closure (mirrors the unix stateless_io_source variant).
    pub(crate) struct IoSourceState;

    impl IoSourceState {
        pub fn new() -> IoSourceState {
            IoSourceState
        }

        pub fn do_io<T, F, R>(&self, f: F, io: &T) -> io::Result<R>
        where
            F: FnOnce(&T) -> io::Result<R>,
        {
            f(io)
        }

        pub fn register(
            &mut self,
            registry: &Registry,
            token: Token,
            interests: Interest,
            fd: RawFd,
        ) -> io::Result<()> {
            registry.selector().register(fd, token, interests)
        }

        pub fn reregister(
            &mut self,
            registry: &Registry,
            token: Token,
            interests: Interest,
            fd: RawFd,
        ) -> io::Result<()> {
            registry.selector().reregister(fd, token, interests)
        }

        pub fn deregister(&mut self, registry: &Registry, fd: RawFd) -> io::Result<()> {
            registry.selector().deregister(fd)
        }
    }
}
