# capsule_tokio_smoke

Runtime gate for the NONOS tokio stack (mio backend + socket2 shim). Runs a
current-thread tokio runtime that races a timer against an idle socket accept;
the timer must keep firing while the accept is parked, which proves the mio
backend's Waker self-wake and the time driver. Emits `[TOKIO-SMOKE]` lines over
serial (Debug capability, mask 0x119).
