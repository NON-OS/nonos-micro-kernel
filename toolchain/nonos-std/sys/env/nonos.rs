// NONOS std env: a process-local environment map. Capsules are spawned
// from signed manifests and inherit no environment block from a parent,
// so the map starts empty; getenv/setenv/unsetenv work within the process
// for the crates that use the environment as configuration. Passing an
// environment across capsule spawns is an open gap, listed with the PAL
// status notes.

pub use super::common::Env;
use crate::collections::HashMap;
use crate::ffi::{OsStr, OsString};
use crate::io;
use crate::sync::atomic::Ordering::Relaxed;
use crate::sync::atomic::{Atomic, AtomicPtr};
use crate::sync::{Mutex, Once};

type EnvStore = Mutex<HashMap<OsString, OsString>>;

static ENV: Atomic<*mut EnvStore> = AtomicPtr::new(crate::ptr::null_mut());
static ENV_INIT: Once = Once::new();

fn get_env_store() -> Option<&'static EnvStore> {
    // SAFETY: the pointer is either null or a leaked Box that lives for
    // the rest of the process.
    unsafe { ENV.load(Relaxed).as_ref() }
}

fn create_env_store() -> &'static EnvStore {
    ENV_INIT.call_once(|| {
        ENV.store(Box::into_raw(Box::new(EnvStore::default())), Relaxed);
    });
    // SAFETY: call_once above published the leaked allocation.
    unsafe { &*ENV.load(Relaxed) }
}

pub fn env() -> Env {
    let vars = get_env_store()
        .map(|store| match store.lock() {
            Ok(map) => map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            Err(_) => Vec::new(),
        })
        .unwrap_or_default();
    Env::new(vars)
}

pub fn getenv(k: &OsStr) -> Option<OsString> {
    get_env_store().and_then(|store| store.lock().ok()?.get(k).cloned())
}

pub unsafe fn setenv(k: &OsStr, v: &OsStr) -> io::Result<()> {
    let (k, v) = (k.to_owned(), v.to_owned());
    create_env_store()
        .lock()
        .map(|mut map| {
            map.insert(k, v);
        })
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "env store poisoned"))
}

pub unsafe fn unsetenv(k: &OsStr) -> io::Result<()> {
    if let Some(store) = get_env_store() {
        store
            .lock()
            .map(|mut map| {
                map.remove(k);
            })
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "env store poisoned"))?;
    }
    Ok(())
}
