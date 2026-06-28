//! NONOS platform backend for relibc. Task 0.2 ships the compiling skeleton:
//! the raw-syscall layer plus the `Sys` carrier. The `Pal` method slice lands
//! in Task 0.3 and the crt0 in Task 0.4.

pub mod lowlevel;

pub struct Sys;
