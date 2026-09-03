# Disk write authority for the installer

The block drivers keep a deliberate fail-closed rule: OP_WRITE_BLOCKS and
OP_FLUSH answer only the kernel-internal client, which arrives as sender
pid 0 because every real capsule's envelope is kernel-stamped with its
pid. No capsule can write a disk today. An installer has to earn that
authority without softening the rule for anyone else.

## The mechanism the kernel already provides

MkProcStat returns, for every live pid, the service name and capability
set the kernel itself recorded at spawn. A capsule only ever holds the
name app.nonos_install because the spawn gate verified its certificate,
manifest, and membership trailer against the baked policy root; the name
is therefore as strong as the attestation chain. The sender pid on an
IPC message is stamped by the kernel and cannot be forged. Joining the
two gives a driver an attested caller identity in two steps and zero new
kernel surface.

## The extended rule

permits(op, sender) for mutating ops becomes: the kernel client as
before, or a sender whose MkProcStat entry names app.nonos_install. The
lookup runs per request; a stale cache would outlive the installer's
exit and hand its pid to a stranger. Everything else stays refused, and
the read-side stays open as today.

## The target device

The store the driver guards at LBA 0 is trusted input on next boot; the
installer must never write the live medium, and the region argument is
not a good enough fence. The install target is a different device: the
machine's own disk, served by its own driver instance. That needs the
one-device-per-capsule drivers to spawn per discovered device
(driver.virtio_blk1, driver.ahci0 on real hardware), which is its own
piece of work. Until it lands, the installer's write step stays pending
and says so; the ACL extension is still built and proven first because
the authority question, not the plumbing, is the security decision.

## The write step itself

With authority and a target: stream /install/nonos.img from the medium
in 64-sector chunks through OP_WRITE_BLOCKS, OP_FLUSH once at the end,
then read every chunk back and compare it against what was sent before
declaring the step done. The image is the same GPT and FAT the usb-img
target produces, so what the installer writes is what the release built
and what the verifier page proved.
