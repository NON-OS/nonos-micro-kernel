// NONOS mio waker. The epoll backend writes an eventfd registered in the
// selector's epoll set; here the selector's wake path is a process global (a
// tokio runtime has one selector), so the waker records its token and raises
// the global flag. An in-flight select() sees it on its next sweep and returns
// with a synthetic readable event carrying the waker token, which is what lets
// tokio break out of a socket poll to service a fired timer or a spawned task.

use std::io;

use crate::sys::nonos::selector::{set_waker_token, signal_wake};
use crate::sys::Selector;
use crate::Token;

#[derive(Debug)]
pub(crate) struct Waker;

impl Waker {
    pub(crate) fn new(_selector: &Selector, token: Token) -> io::Result<Waker> {
        set_waker_token(token);
        Ok(Waker)
    }

    pub(crate) fn wake(&self) -> io::Result<()> {
        signal_wake();
        Ok(())
    }
}
