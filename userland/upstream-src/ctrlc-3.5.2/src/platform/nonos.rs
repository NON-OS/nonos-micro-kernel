// NONOS platform for ctrlc. On NONOS the terminal (nox) owns Ctrl-C and
// delivers interrupts to the foreground job itself, so a capsule installs no OS
// signal handler. init_os_handler is a no-op and the crate's waiter thread
// parks here, which lets ctrlc build and link for x86_64-nonos without the unix
// signal syscalls the target does not have.

/// A platform error. NONOS raises none of these, but the crate wraps it into an
/// io::Error and compares it to EEXIST, so it carries an errno-like code and
/// implements the traits that requires.
#[derive(Debug, PartialEq, Eq)]
pub struct Error(pub i32);

impl Error {
    /// The "already installed" code the crate checks to detect a second handler.
    pub const EEXIST: Error = Error(17);
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "nonos ctrlc error {}", self.0)
    }
}

impl std::error::Error for Error {}

/// A platform signal value, carried in `SignalType::Other`. NONOS has no OS
/// signal numbers, so a plain integer stands in.
pub type Signal = i32;

/// Install the OS Ctrl-C handler. On NONOS the terminal delivers interrupts, so
/// there is nothing to install.
///
/// # Safety
/// Matches the other platforms' `unsafe` contract; this touches no OS state.
pub unsafe fn init_os_handler(_overwrite: bool) -> Result<(), Error> {
    Ok(())
}

/// Block until Ctrl-C. On NONOS the signal never reaches the capsule, so the
/// caller's waiter thread parks here indefinitely instead of spinning.
///
/// # Safety
/// Matches the other platforms' `unsafe` contract; this only parks the thread.
pub unsafe fn block_ctrl_c() -> Result<(), Error> {
    loop {
        std::thread::park();
    }
}
