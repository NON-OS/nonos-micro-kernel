# nonos_qjs: a no_std JavaScript engine for the browser capsule

This crate embeds QuickJS-ng as a freestanding, no_std JavaScript engine and
binds it to the browser's DOM. It exists because the browser's own tree-walk
interpreter, while good for light dynamic pages, cannot run production framework
bundles: React, Vue and Svelte lean on Symbol, getters and setters, Proxy,
prototypes, typed arrays, precise microtask timing and ES modules, and they are
performance sensitive. Reimplementing all of that as an interpreter is a
multi-year engine project. Embedding a complete engine is not.

The engine question people assume cannot be answered in no_std is answered here:
a full ES2020+ engine compiles, links, and drives the DOM inside a sovereign
capsule, with no libc and no operating system underneath it.

## Layers

```
  page <script>  ->  qjs_run::run_scripts  ->  Engine (this crate)
                                                  |
                          QuickJS-ng core (C, compiled freestanding)
                                                  |
                          C stub layer  +  Rust stub layer
                                                  |
                          Rust global allocator, libm crate, compiler_builtins
```

- `Engine` (src/engine.rs) is the safe Rust surface: `new()`, `eval()`,
  `install_dom(host)` and `dispatch_event(node, type)`. Values never cross the
  FFI boundary; only handles and strings do.
- The C core (vendor/) is QuickJS-ng: quickjs.c, libregexp.c, libunicode.c,
  dtoa.c. It is compiled freestanding by build.rs.
- The DOM glue (vendor/dom_bindings.c) and the eval shim (vendor/eval_shim.c)
  wrap the parts of the QuickJS API that are inline in the headers and so cannot
  be called from Rust directly.
- The stub layer supplies the C-ABI symbols the freestanding core references.

## The freestanding build

build.rs cross-compiles the C core with clang for a bare x86_64 target:

```
clang --target=x86_64-unknown-none -ffreestanding -fno-stack-protector \
      -fno-builtin -mno-red-zone -fPIC -Dalloca=__builtin_alloca -nostdinc \
      -isystem shim -isystem <clang-resource>/include -I vendor -O2
```

QuickJS assumes a hosted C library, so shim/ provides minimal headers declaring
exactly the symbols it uses (stdlib, string, stdio, math, pthread, time, and a
few more). Nothing in shim/ implements anything; it only declares. `-fPIC` is
required because the browser capsule links as a position-independent
executable.

## The stub layer

The freestanding core resolves to about seventy external symbols, and almost all
of them cost nothing:

- libm (sin, cos, pow, exp, log, fmod, and the rest): forwarded to the pure-Rust
  `libm` crate in src/math_stubs.rs.
- allocator (malloc, calloc, realloc, free, usable_size): src/alloc_stubs.rs
  over the Rust global allocator, with a header word per block carrying its size.
- mem routines (memcpy, memset, memmove, memcmp): from compiler_builtins.
- string routines (strlen, strchr, strcmp, strstr): src/str_stubs.rs.
- abort and a decimal strtod: src/misc_stubs.rs.
- pthread (single-threaded no-ops, since a capsule runs one context), zeroed
  time sources, and a minimal integer/string vsnprintf: vendor/stubs.c.

The only hand-written work of any weight is vsnprintf; everything heavy is
already in the dependency tree.

## The DOM bindings

A DOM element is a JS object carrying a hidden `__node` integer, its index in
the host DOM. Every mutating method and property accessor routes through an
`njs_dom_*` callback that the host (the browser capsule) implements over its
real node tree. The host pointer travels as the QuickJS context opaque, so the
callbacks reach the tree without globals.

Bound today:

- document.createElement, getElementById, querySelector, querySelectorAll (the
  last two backed by the browser's own css::select), and document.body.
- element.appendChild, removeChild, setAttribute, getAttribute, innerHTML
  (parsed and grafted with the browser's HTML parser).
- element.textContent (get and set), className (get and set), id, tagName.
- element.style.<prop> = <val> through a Proxy set-trap, accumulating into the
  node's inline style, with camelCase keys converted to kebab-case.
- element.addEventListener and event dispatch: listeners are held with a
  duplicated function reference so they survive between the script run and a
  later UI event; dispatch fires them with an event object and drains the job
  queue.

## The page executor

`qjs_run::run_scripts(dom)` collects a page's `<script>` elements, runs them
through QuickJS with the DOM installed, and returns the engine. Because the
engine holds the page's listeners and closure state, it is returned rather than
dropped, so later UI events can be dispatched into it. The caller lays the
mutated tree out afterwards, which is the reflow.

The browser wires this in at page commit. `fetch::render_response` now hands
back the parsed DOM alone; the page state homes it in `state.page_dom` and then
`fetch::commit_html` runs its scripts through QuickJS against that stable
address, keeps the engine in `state.engine`, and lays the mutated tree out. The
engine holds a pointer into `page_dom`, which keeps its address for the page's
life, so a navigation drops the engine before it replaces the DOM. A click, a
form field edit or a submit dispatches straight into the retained engine and the
page relays out, so a script that mutates the tree in response to input is
reflected on screen. QuickJS is the browser's only script executor; the earlier
tree-walk parser and interpreter are gone.

## Verification

Every layer is proven on the host, without QEMU, the same way the tree-walk
features were:

- The engine evaluates the full ES2020+ surface (classes, generators, spread,
  destructuring, optional chaining, async, regex, template literals).
- The DOM bindings build a real tree from script, including a framework-style
  `Component(mount, data)` render producing a correct nested list.
- Events fire with closure state preserved across dispatches.
- Inline styles accumulate correctly with camelCase conversion.

The crate compiles for x86_64-nonos-user, links into the browser capsule, and
the archive carries the QuickJS symbols. It also runs on the real kernel: a
startup probe that created a runtime and evaluated an expression ran to
completion on hardware with no fault, which is how the input-termination and
stack-anchor fixes below were confirmed. The probe has since been retired in
favour of the live render path.

## The freestanding fixes

Two defects only show up in the freestanding, on-device build and were fixed
against the engine's own contract:

- `JS_Eval` requires its input to be NUL terminated (`input[len] == '\0'`). The
  Rust caller passes a `&str`, which is not, so the eval shim now copies the
  source into a terminated buffer before handing it to the engine. Without it
  the lexer reads past the source and faults on a page boundary.
- The runtime captures its stack top when it is created, and the browser creates
  the engine far above the frame a page script later runs from. The shim
  re-anchors the stack top before each eval and each event dispatch, so a
  shallow script cannot trip a false stack-overflow.

## Honest status and remaining work

The engine is the browser's script executor: page scripts run through QuickJS,
the engine is retained for event dispatch, and the tree relays out after input.
The engine, the DOM write and query surface, events and inline style are done,
building and running on the kernel. What remains to render an arbitrary React or
Svelte site:

- External `<script src>` bundles are not fed to the engine yet; only inline
  scripts run. Most framework apps ship an external bundle, so fetching those
  into the engine is the next step.
- setTimeout, setInterval and fetch/XHR are not bound to the host loop yet, so a
  standalone microtask pump and a network binding are still to come. The browser
  keeps its socket-level fetch and timer pumps as the scaffolding these will use.
- Node identity, so getElementById returns the same object twice.
- WebGL, WebAssembly and Web Workers remain out of scope, so canvas-and-shader
  pages will not fully render.

The ceiling is honest, but the DOM-based majority of the modern web, including
framework single-page apps once their bundle is fed in, is now reachable.
