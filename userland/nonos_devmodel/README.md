# nonos_devmodel

Driving a real driver's register code with no device attached. `FakeBar` is
the window, `run` puts a model on a second thread against it.

## What a concurrent model can and cannot reach

It reaches anything the driver **waits** for. A reset handshake that writes
a bit, spins until the device echoes it, clears it, and spins until the
device drops it is the case this exists for, and it is one a passive window
can never satisfy: whichever value you preload, one of the two waits spins
forever. Here the model simply answers each edge in turn.

It does not reach a register the driver writes and reads back in the next
instruction. The virtio feature negotiation is the example: `w8(STATUS,
FEATURES_OK)` followed immediately by `r8(STATUS)` leaves a model thread
nanoseconds to intervene, and it will usually lose. Modelling a device that
reacts *within* a single write-then-read needs the write itself intercepted,
which raw volatile stores to memory do not allow, and faking it with a
timing-dependent race would produce a test that passes most of the time,
which is worse than no test.

So the rule for choosing a model: if the driver spins, a live device can
answer it. If the driver reads back immediately, it cannot, and the property
stays documented rather than asserted.

