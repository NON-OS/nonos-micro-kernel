use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use unix::*;
        pub(crate) use rustix::fd::AsFd as AsOpenFile;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::*;
        #[doc(no_inline)]
        pub(crate) use std::os::windows::io::AsHandle as AsOpenFile;
    } else if #[cfg(target_vendor = "nonos")] {
        mod nonos;
        pub use nonos::{RwLock, RwLockReadGuard, RwLockWriteGuard};
        pub(crate) use nonos::AsOpenFile;
    } else {
        mod unsupported;
        pub use unsupported;
    }
}
