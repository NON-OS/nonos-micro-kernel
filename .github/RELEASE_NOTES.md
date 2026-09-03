# NONOS 0.9.2 (pre-release)

506 commits between August 7 and September 3, written by
[@eKisNonos](https://github.com/eKisNonos) and
[@senseix21](https://github.com/senseix21). This document covers all
of it: what broke, how we found it, what is true now, and why this
tag is marked pre-release even though it is the most complete NONOS
to date.

## Contents

- [The shape of the month](#the-shape-of-the-month)
- [Rebuilding the release process](#rebuilding-the-release-process)
- [The reply that never came](#the-reply-that-never-came)
- [Scheduler and SMP](#scheduler-and-smp)
- [Doors into the kernel](#doors-into-the-kernel)
- [The IOMMU starts translating](#the-iommu-starts-translating)
- [Capabilities, syscalls and the SDK contract](#capabilities-syscalls-and-the-sdk-contract)
- [Attestation](#attestation)
- [Crypto and TLS](#crypto-and-tls)
- [Packages with consent](#packages-with-consent)
- [Repainting the desktop](#repainting-the-desktop)
- [The applications](#the-applications)
- [Networking](#networking)
- [Drivers](#drivers)
- [The installer](#the-installer)
- [Why 0.9.2 is a pre-release](#why-092-is-a-pre-release)
- [Verifying this release](#verifying-this-release)

## The shape of the month

NONOS is a capability-based microkernel where every capsule that
runs is signed, measured and attested, and where the goal is that
nobody, including us, has to be believed about anything. The month
since 0.9.1 pushed that idea into places it had not reached yet: the
release pipeline itself, the capability paper trail, the package
install path, and the installer that will eventually put the system
on a disk.

The raw numbers: 3,316 files touched, about 74,600 lines added and
25,800 removed. The commit histogram is dominated by the terminal
and document editor arcs, but the changes that matter most are
smaller and deeper: the IPC reply path, the kernel entry code, the
IOMMU, and a release process that now proves what it ships.

## Rebuilding the release process

This is the story of why 0.9.2 took a week longer than planned and
why that week was the most useful one of the month.

Our release gates used to run only on tags. That sounds economical
and it is, in the way of not testing a fire alarm because fires are
rare. When we pointed the gates at the tree in late August, four
defects surfaced that had been sitting there for weeks, invisible
because nothing ever exercised them.

The first was the trust ledger. Back in July a keystore
re-enrollment replaced all 371 trust artifacts, but the commit
carried the previous MANIFEST.sha256. Every artifact was valid,
every signature checked out, and the ledger verification still
refused 295 of them on any clean checkout, because the ledger
described the set that had just been replaced. Nobody noticed for
six weeks because nobody's workflow ran the check.

The second was the reproducible-build lane, which is supposed to
build the kernel twice from two clean checkouts and compare them. It
had never completed a single run. It built from git worktrees, and a
worktree mounts submodules as empty directories, so every attempt
died on the first missing include, before ever reaching the
comparison it existed to make.

The third was small but instructive: host tools were cached behind a
stamp file, so a signing tool binary could outlive arbitrary changes
to its own sources. All five host tools now depend on their real
source trees, found at make time, so editing a tool means running
the edited tool.

The fourth was the serious one. Production build lanes re-signed
every capsule as part of the build. That has two consequences and
both are wrong. It means CI needs signing seeds, which is exactly
the material a build pipeline must never hold. And had it ever
succeeded, it would have quietly replaced the owner-enrolled
artifacts with CI-minted ones, which inverts the entire trust model:
the pipeline is supposed to verify what the owner enrolled, not
mint its own truth.

### Trust-reuse: the build that signs nothing

Fixing the fourth defect meant deciding what a production build
actually is. The answer we landed on: a production build is a check,
not a ceremony. In trust-reuse mode the build performs zero signing
operations. The committed policy, certificates, manifests and
membership trailers must verify under the trust anchor baked into
the kernel, and every freshly compiled capsule must hash to the
payload recorded in its enrolled manifest. The signing tool grew a
new check for exactly this, `verify-manifest --elf`, which binds a
manifest to the bytes of one ELF and, on mismatch, prints both
hashes so the drift is visible rather than mysterious.

Across the shipping set that is 87 payload verifications per build.
If a capsule's rebuild drifts from its enrolled measurement, the
build fails naming that capsule, and the only way forward is to make
the capsule reproducible. There is no code path in a production
build that can create a signature, so there is nothing for a
compromised runner to mint.

### The enrollment ceremony

Trust-reuse raises an obvious question: reproducible against what?
Until now the enrolled set was whatever the working tree produced,
which means the reference point for all reproducibility was a build
environment nobody else could reproduce.

Enrollment is now split across two machines with two different
privileges. A dispatch workflow, `build-for-enrollment`, compiles
the complete capsule set on the pinned Linux toolchain under the
deterministic build environment, then publishes the ELFs together
with a BLAKE3 manifest, the exact commit and the toolchain record.
That machine holds no secrets and signs nothing; its only authority
is being boring and repeatable.

The owner side runs where the seeds live. A small tool walks five
steps: load the builder artifact and check it matches the tree,
verify every binary against the builder's BLAKE3 manifest, place the
binaries, run the ordinary signing flow, and then, before the ledger
is restamped, re-verify that all 87 binaries came through signing
byte-identical. That final check is not decoration. During this
release it fired: a stale dependency edge caused the build system to
rebuild one capsule in the middle of the signing pass, and the
ceremony refused to enroll the swapped bytes. Without the re-verify,
the keystore would have pinned a binary the builder never produced,
and every downstream reproducibility claim would have been false in
a way nobody would have caught for months.

We also ran the ceremony twice for a reason worth recording. The
about and terminal capsules embed the VERSION file at compile time,
so bumping the tree from 0.9.1 to 0.9.2 changed their bytes, which
changed their measurements, which invalidated the enrollment made
before the bump. Version bumps come before enrollment, always. It
is obvious in retrospect, which is the only place it is obvious.

### Byte-for-byte, and where byte-for-byte is impossible

The kernel ELF was already reproducible: two independent clean
checkouts produce identical bytes on the pinned toolchain. The
bootloader was not, and the diff told two separate stories. Absolute
checkout paths were reaching the binary through panic location
strings, and the PE debug directory carried a PDB path no UEFI
image has any use for. With the repository root remapped through
`--remap-path-prefix` and the debug directory dropped with
`/DEBUG:NONE`, the loader now rebuilds byte-identical too. Release
profiles build with a single codegen unit, and every timestamp that
reaches an image is pinned.

The signed and attested images will never be byte-identical, and
saying otherwise would be a lie with a countdown attached: ML-DSA
signatures and STARK proofs are randomized on every run by design.
So the double-build lane compares what is supposed to match, the
measured payload, and verifies what is supposed to verify, the
signatures and proofs, instead of diffing bytes that were never
going to agree.

### The image nobody owns yet

Release images used to bake the owner's device binding root, which
meant the published image carried binding material belonging to one
specific owner. In a stark-attested build the device
root is not a boot gate; the kernel's self-attestation is. The
device root only backs runtime binding proofs. So the vendor image
now bakes a public sentinel instead: the SHA-256 of the string
`NONOS-DEVICE-SLOT-UNBOUND-v1`. Anyone can derive it, it authorizes
nothing, and every binding proof offered against it is refused.
That is the correct behavior for hardware nobody owns yet. A
machine becomes someone's at install time, when the installer
enrolls real device slots, and not one second before.

### What guards the gate now

All of these checks, the trust ledger, the sentinel pin, the
double-build comparison and the full production build, now run on
every pull request and every push, so the class of defect that
started this section cannot go dormant again. Cutting a release is
a single dispatch that inspects every check-run on the target
commit and refuses to move the tag unless all of them are green.
And since the ceremony rework touched every signature anyway, this
release rotates all 87 publisher keypairs in both algorithms,
Ed25519 and ML-DSA-65, with certificates reissued under the
unchanged trust anchor. CI additionally gained a Windows lane that
boots the image under VirtualBox from a release asset, because
"boots on the machines we own" is not the same claim as "boots".

## The reply that never came

For most of August the worst bug in the tree looked like three
different bugs. A capsule would call a service and hang. Sometimes
the crypto capsule, sometimes nym, sometimes the vfs, never
reliably, never with a trace. The common thread turned out to be
the reply path: replies were paired to callers by queue position,
so any timeout, any self-served endpoint, any kernel-mediated
reply landing in the wrong inbox shifted the pairing by one, and
every reply after it went to the wrong caller or nowhere.

The fix was to make correlation explicit. Every call carries a
token and a reply only delivers to the caller holding that token.
A reply whose token matches no waiting caller is dropped and
flagged by the diagnostics instead of poisoning the queue, and
pairing stays aligned when a call times out. Kernel-mediated
replies are now delivered where the kernel actually drains them,
which sounds tautological until you learn they previously were not:
one commit in the chain is literally titled "put a kernel-mediated
reply where the kernel drains it". Service replies reach the
caller's reply inbox instead of its process inbox. A spawned
capsule's reply inbox is claimed for its owner at spawn instead of
sitting unowned. Redirect replies issued by the kernel itself now
pass the endpoint capability gate that used to reject them, which
was the single change that brought the crypto, nym and network
capsules back from the dead after the original reply-drop
regression. And a reply aimed at a self-served endpoint is dropped
deliberately, because delivering it would hand a service its own
message back as fresh work.

The reason these bugs survived so long is that a dropped reply is
silent by nature. So the trust boundaries got rate-limited
diagnostics: when the kernel refuses or drops something, it says
so, at a rate that cannot be weaponized into log flooding by a
misbehaving caller.

## Scheduler and SMP

The scheduler's wake path was rebuilt around a lock-free wake
generation table. Wakes are counted, so a task that was woken while
preparing to sleep can refuse to sleep through the wake it would
otherwise have missed; the receive path and the irq-wait path both
guard their sleeps with the wake token. This closed the classic
lost-wakeup race in the two places capsules actually block.

SMP bring-up collected three fixes that matter long before
multi-core ships by default. Secondary CPUs come up without racing
the bootstrap processor. An application processor now loads the IDT
the kernel actually runs, rather than whatever the early boot table
happened to be. And the cpu number is read from the per-cpu block
instead of a register that is not architecturally guaranteed to
hold it. Shipped profiles remain single-CPU, deliberately, until
the remaining races are proven out; the fixes above are why that
proving is now possible at all.

Two small things in the same neighborhood: the boot splash hands
off based on uptime rather than the wall clock, so a machine with a
wrong RTC does not sit on the splash forever, and `SYS_PROC_STAT`
now reports total memory and load average, which is what the
terminal's telemetry rail displays.

## Doors into the kernel

Every way into the kernel is now hand-written assembly, one file
per concern: the exception trampolines, the page fault and timer
entries, and the 56 numbered vector gates with their shared
dispatch body. The point is reviewability. Questions like "does
swapgs run on this path" or "is the FPU area saved before the first
use" are answered by reading one short file top to bottom, not by
trusting a macro expansion.

That review posture immediately paid for itself:

- `swapgs` now runs on every vector a capsule can enter the kernel
  through, not just the common ones.
- `sysretq` rejects every non-canonical return RIP. The previous
  check covered only half the non-canonical range, and a
  non-canonical RIP at sysretq is a classic privilege escalation
  primitive on Intel hardware.
- Every fault stack moved out of the kernel image into its own
  allocation with a guard page below it, so a fault-stack overflow
  faults loudly instead of silently corrupting whatever the linker
  placed next to it.
- The IRQ layer refuses to hand a capsule any interrupt line the
  kernel itself listens on.
- Speculation mitigations are applied, and applied on kernel entry,
  which is the only place they mean anything.
- The ring-0 MMU restrictions are put in force and then read back,
  so a firmware or virtualization quirk that silently ignored the
  write would be caught at boot instead of discovered during an
  exploit post-mortem.

## The IOMMU starts translating

Until this cycle the IOMMU code reported that translation was off
and left it off. That deserves to be said plainly because the
capability model makes a specific promise: a driver capsule gets
the devices it is granted and nothing else. Without the IOMMU
enforcing that promise at the bus, a compromised device or a
malicious DMA engine ignores the capability system entirely.

The VT-d path now performs real translation with real invalidation
and real fault reporting. The fault queue is drained from the timer
tick, so a fault posted by a device shows up in diagnostics instead
of rotting in a ring buffer. Enforcement is on. This is the
difference between a security architecture that is described and
one that is turned on.

## Capabilities, syscalls and the SDK contract

A capability system is only as honest as its paper trail, and ours
had drifted. This cycle audited it end to end and then built the
machinery to keep it from drifting again.

The capability table is now complete and its mirrors agree. Every
capability constant that was not backed by a real kernel bit is
gone; several services were gating on bits the kernel never
checked, which is security theater of the purest kind. Service
gates bind to real kernel bits, and the IPC layer refuses to
register an endpoint that declares no capability requirement at
all, so "forgot to specify" fails loudly instead of defaulting to
open. A parity check runs under make before anything is built or
signed, comparing the table against the kernel, and the published
documentation table is checked against the kernel the same way.
The Lean capability extraction was extended to track the same
table, including the StoreWrite capability and the diagnostic
tables, so the formal model and the enforced reality cannot
quietly diverge.

The syscall surface got the same treatment. The published ABI now
lists the syscalls the kernel actually dispatches; 42 were missing
from the published file, and the attestation and dev-root syscalls
are now documented rather than folklore. The SDK requires an
application to declare what it may do, and those declared
capabilities travel with the signed artifact, which is what the
package consent screen reads back to the user later. And libc
dropped the graphics wrappers for calls the kernel no longer
serves, so the compatibility surface stops advertising ghosts.

## Attestation

The STARK engine moved into its own repository, `stark-attest`,
and everything consumes that one crate: the kernel's spawn gate,
the bootloader, the host tooling, the verification pipeline. Before
the extraction there were multiple copies at multiple ages, and the
kernel's copy was the stale one, which is exactly backwards. Now
the prover that gates boot is the prover that verified the release.

Around the engine:

- The build ends by proving what it produced; the attestation
  surface bundle ships with every tag and feeds
  verify.nonos.software, covering the boot chain, the kernel and
  all 87 capsules.
- Periodic columns are evaluated by the subgroup closed form
  instead of table lookups.
- The attestation round count is read from one place instead of
  three, and CI now forbids any local copy of a soundness
  parameter, because a soundness parameter that exists twice is
  one commit away from existing with two values.
- The Lean side discharged context injectivity for the attestation
  layout, recorded in the evidence corpus: two distinct attestation
  contexts cannot serialize to the same bytes.
- The kernel keeps a registry of what is actually running and can
  export a signed attestation document over the TPM, so "what is
  this machine running" has a cryptographic answer, not a
  chat-window answer.
- The new image-ceiling and local-build modules let code signed on
  this machine run without weakening the check for anything else,
  and dev-root consent codes are drawn from hardware entropy, so
  granting a development root requires physical presence in front
  of the machine that generated the code.

## Crypto and TLS

The crypto layer had a panic path in the random generator; in a
kernel that is a denial-of-service primitive, and it is gone.
Random is served from a capsule-seeded generator. x25519 now
rejects non-contributory shared secrets, the class of weak-point
inputs that let an attacker force a known shared key.

TLS grew three checks that each close a real attack. Certificate
basic constraints are checked before trusting an issuer, so a leaf
certificate cannot act as a CA. ISRG Root X2 is pinned, and the
clock that certificate validation runs against is traced in
diagnostics, because certificate validation against a wrong clock
is validation of nothing. And a server flight is only treated as
complete on a clean record boundary, closing a truncation window
where a cut-off handshake could be mistaken for a finished one.

## Packages with consent

This cycle NONOS installed third-party software for the first time,
and the path it took is worth walking through, because every step
was designed around a refusal.

The vfs grew a dedicated set of install operations for the capsule
store: `OP_PKG_QUERY` returns a verified summary of a package,
`OP_PKG_COMMIT` installs into `/capsules` only after the full
artifact chain verifies, and `OP_PKG_REMOVE` removes installed
artifacts. All of them, along with store-remove, are restricted to
the installer service; no other capsule can touch the store, and
the store itself is read-only outside these ops. The store enforces
its budget, raised to 16 MiB with a 48 MiB heap behind it, and
refuses silent overwrites. The install slug is derived from the
package's verified namespace rather than anything the package says
about itself, and a slug that an existing service already answers
is refused, which closes the squatting move where a package names
itself after a system service.

On the desktop, the shell scans `/pkgs` every tick and surfaces
installable packages as a Launchpad row. Tapping one raises a
consent modal that always shows the package's requested
capabilities by name, from a display-name table, before the approve
button commits anything. The modal never hides capabilities to
look friendlier, and install progress is shown rather than implied.
A corrupted store surfaces as a toast instead of a silent absence.
The terminal got the same power in text form: `nox pkg` install,
remove and status with a capability decoder that names unknown
bits rather than swallowing them.

The first package through the pipe was a QR generator capsule built
from the unmodified crates.io `qrcode` crate: unaltered upstream
Rust, compiled for NONOS, signed, verified, consented to, and
installed. That is the app story we are building toward, standard
Rust from the ecosystem running as attested capsules, and it works
today at small scale.

## Repainting the desktop

The desktop went from functional to something we stopped being
embarrassed by, and the interesting part is that almost none of it
was app work. It was infrastructure.

The toolkit gained a paint primitive layer: antialiased rounded
rectangles, panels, circles and rings, linear gradients, soft drop
shadows, plain and antialiased lines, clipping sub-views, and alpha
compositing, built on fixed-point corner coverage arithmetic and
covered by host tests that assert frame geometry. The window frame
itself became rounded, shadowed chrome owned by the app skeleton's
coordinate space. Blending preserves destination alpha, and the
compositor blends two colour channels per multiply, which cut the
per-pixel cost of the single hottest loop in the system.

Text stopped being 1-bit. The shell's bitmap text path was retired
and everything, menu bar, Launchpad tiles, desktop icon captions,
toasts, menus, consent dialogs, draws through an antialiased text
facade. A feature-gated frame-time counter went in first, so the
restyle was measured rather than felt.

The shell learned about display density. A scale factor is derived
from the surface size and every metric follows it: the dock, both
icon grids, context menus, toasts, consent dialogs, the menubar
brand, badge stairs, frame extents, row heights at the measure
boundary. Icon art comes from one shared SVG source through an
alpha-mask generator, drawn as tinted masks and resampled by exact
area coverage, so the dock, the desktop, settings and the brand
mark are one set of assets at every size instead of four sets of
approximations.

Underneath, virtio-gpu negotiates the EDID feature, queries
GET_EDID, and reports the panel's physical size at bring-up, which
is what makes the scale factor honest. It also gained 3D resource
support with a real command stream. And the QEMU run path picked up
a QMP control socket and a VNC display option, with the bootloader
reporting the granted GOP mode on serial, which together turn
display debugging from archaeology into observation.

## The applications

**Terminal.** The month's largest single arc, roughly sixty
commits. It now has a real layout module, pure and host-tested,
that computes every window region; block-based output where each
block is headed by the session context that produced it; a left
rail with sessions and projects; a right rail with live telemetry,
load, memory and disk, fed by `SYS_PROC_STAT`; a command palette;
tabs living in the titlebar accessory; and a shell that starts in a
real home directory. Themes carry real palettes with a test that
asserts they stay legible, so a restyle cannot quietly produce grey
on grey. Theme and zoom persist across reboots through a versioned
prefs record. The coreutils stopped pretending: ls gives a full
long listing, and grep, head, tail and sort take their real flags
through a shared short-option parser, with sort growing -n, -r and
-u. Neofetch is there too, laid out in two columns, because an OS
that cannot show off is not finished.

**NONOS Docs.** The text editor grew into a document editor and
took a new name on the way. Underneath is a real document model,
blocks containing styled runs, with insertion and deletion that
preserve and merge runs, all mutation through one funnel, and
layout through measured line boxes built from actual TTF metrics.
Pagination follows the caret, not the scroll position, and clicks
hit-test against the very line boxes the painter drew, so the
caret lands where the eye says it should. On top of that:
a menu bar, a formatting ribbon, paragraph alignment, tables,
lists, a settings screen with real panels, and a Docs home screen
with document rows. Documents export to Markdown, DOCX and PDF,
reachable from Ctrl-E, through a stored-entry ZIP writer built for
the purpose. The toolkit learned to render sheared glyph runs so
italics are actually italic. The layout engine has its own host
test suite, and the arithmetic that positions everything is
clamped against overflow.

**Video player.** New, and an MVP in the honest sense: every layer
exists and the whole chain works. A RIFF chunk iterator with
bounds-checked byte helpers, AVI header and stream parsing, a frame
index built by offset-base probing, a seeking stream reader over
the vfs for windowed file access, frame decoding, letterbox
scaling, a playback clock with a host-tested time formatter, a
transport bar with keyboard mapping and click routing, and a
library screen that discovers playable media in the vfs. The AVI
parser is tested against both the real fixture and hostile input,
polling drops to idle while paused instead of spinning at 4ms, and
zune-jpeg is proven to build AVX2-free for the user target. There
is a boot smoketest whose capsule trailers verify with the STARK
reader, so even the demo path goes through attestation.

**Process manager.** Rebuilt from a two-view tool into a six-screen
model routed through one dispatch: process table, inspector,
authority matrix, and three more, with filter chips, search, sample
history and sparklines. Its distinguishing test: painter and
hit-test geometry are asserted to agree on the host, so a click
cannot land on a row the paint code did not draw.

**Calculator.** Rebuilt around modes: standard, scientific,
programmer, unit conversion and history, with a mode rail, its own
icon set, and unit conversion folded into a single fraction so it
does not accumulate rounding across steps.

**Settings, Snake, About.** Settings was restyled around a sidebar
and grouped cards drawing from the shared icon source. Snake became
a six-screen arcade with a FileSystem-backed run store, which makes
it, absurdly but genuinely, a useful test of the persistence path.
About was rebuilt on the toolkit and reads its uptime from the
monotonic clock and its version from the tree, which is the file
whose embedding forced the second enrollment ceremony.

**Desktop shell.** The Launchpad got a title, a search pill, page
dots, and a resolved visible set that paint, click and type all
share, so what you see is literally what hit-testing sees. The
menu bar has working drop-down menus and a dated clock, desktop
folders say how much they hold, the dock blends and rounds instead
of punching a hole in the wallpaper, and the shell survives a
market service that registers and then never answers, which
previously held the entire desktop hostage on boot.

## Networking

The nym mixnet path stopped assuming the network is polite. Replies
arriving interleaved are reassembled through a pooled collector
instead of one at a time. The gateway list is fetched with retry,
and a directory sync refuses to call itself done without one; the
exit list gets the same retry, and the client keeps re-syncing
until the directory actually carries an exit, because a directory
without exits can technically sync and practically route nothing.
Chunked transfer framing is stripped off the directory fetch, the
bootstrap gateways were refreshed, and when a step refuses, the
diagnostics now say which step, instead of reporting only that
refusal occurred somewhere.

SOCKS5 answers the requests it cannot parse instead of leaving the
client hanging on a dead socket, holds its proofs to the reply code
rather than to a boolean, and its relay poll window was tuned twice
until mixnet replies are caught while the browser stays responsive.
The net-core serve thread is bounded so client calls fail fast
instead of timing out into mystery. And the vfs replies to the
capsule that called it rather than the kernel inbox, one more
member of the reply-path family documented above.

## Drivers

The virtio block driver stopped burning cycles and started telling
the truth: it blocks on its interrupt instead of yield-polling,
resyncs its used-ring cursor after a timeout instead of wedging
permanently, ignores its own reply loopback in the serve loop, and
reports driver setup on the serial log.

The RTL8821CE WiFi story is a lesson in error attribution. The
symptom was an efuse read failure on real hardware. The actual bug:
a station MAC draw had been routed through a crypto-gated syscall,
and the driver had never been granted the Crypto capability, so the
capability system was refusing it, correctly, while the error
message blamed the efuse. The capability is now granted where it is
needed, the driver refuses an efuse map the chip never actually
answered rather than parsing garbage, and failures name the step
that failed. Six wrong hypotheses got ruled out before the right
one; the diagnostics exist so the next person needs zero.

virtio-gpu's grants were straightened out along the way: Debug
arrives through optional capabilities rather than required ones,
the id-cert ceiling spans both capability sets, and the desktop
shell dropped the Debug capability it never needed. Least privilege
is maintained by exactly this kind of tedium.

## The installer

`capsule_nonos_install` is written in assembly, every instruction
from `_start` to the exit syscall, and it goes through the same
signing, enrollment and spawn-time attestation as every other
capsule; the spawn gate treats it as nothing special, which is the
point. It exists to walk the install ritual on a live medium, and
three of its seven steps work today.

Step one surveys the machine through the hardware broker. Step two
reads the install set off the medium over the ramfs wire protocol,
through a client that bounds-checks lengths, offsets and handles on
everything the server returns, because the file server is outside
the installer's trust line. Step three drives every named capsule's
four trust artifacts, binary, certificate, manifest, membership
trailer, through the same verification chain the spawn gate uses,
and prints each capsule's granted capability mask beside its
verdict, so the operator sees the authority being installed, not
just the fact of installation. A single verification failure aborts
the ritual. The failure mode is refusing to install; there is no
path that installs less and reports success.

Steps four through seven print themselves as pending, and honestly
naming them is most of why this tag says pre-release. Minting the
machine's attestation root and enrolling the composed system need
the local-build root service exposed to capsules behind a
physical-presence consent step. Writing the boot partition needs
two pieces that are specified but not built: per-device driver
instancing, so the target disk gets a service of its own, and the
write-authority ACL that admits exactly the installer through the
block driver's otherwise-total refusal to write. The install
receipt, which will tie the finished machine to its public
verification page, lands with those.

## Why 0.9.2 is a pre-release

- The installer verifies but cannot yet mint a root, write a disk
  or produce a receipt. An installer that cannot install is a
  verifier with ambitions, and we will not call it more than that.
- The documentation program is mid-rewrite: per-module deep dives,
  wire formats byte by byte, build guides with real transcripts,
  everything checked against the code it describes. It is
  substantial and it is not merged.
- SMP boots multi-core in bring-up, but every shipped profile is
  single-CPU until the remaining races are proven out.
- Real-hardware driver coverage is NVMe, AHCI, xHCI, PS/2 and
  RTL8821CE. Everything else is exercised under QEMU, and we do
  not count QEMU as hardware.
- The aarch64 tree compiles and boots as a preview and is not a
  signed target.

Each of these has active work behind it, and none of them changes
what the tag already delivers: a reproducible, verifiable release
of everything described above.

## Verifying this release

Nothing in this document asks to be believed. The attestation
surface bundle is attached as a release asset and served at
verify.nonos.software, covering the boot chain, the kernel and all
87 capsules of this tag. The kernel and bootloader rebuild
byte-identical from this commit on the pinned toolchain. The
capsule set rebuilds to the measurements in the committed trust
ledger, and the unbound sentinel derives from a string printed in
plain text above. Check any of it.
