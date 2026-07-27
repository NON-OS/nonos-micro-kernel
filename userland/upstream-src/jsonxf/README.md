# Jsonxf

A JSON transformer, written in Rust.

Provides fast pretty-printing and minimizing of JSON-encoded strings
and streams, at the command line or within Rust programs.

Jsonxf is built for speed, and does not attempt to perform any
input validation whatsoever.  Valid input produces valid output,
but no guarantees are offered around the detection and rejection
of invalid input.

Crate docs: https://docs.rs/jsonxf/


## Installation

    cargo install jsonxf


## Command-line Examples

Pretty-print a string to the terminal, using two spaces to indent:

    jsonxf -s '{"a": {"b": 2, "c": false}}'

Pretty-print and read a JSON file, using a tab character to indent:

    jsonxf -t $'\t' <foo.json | less

Minimize a file and gzip it:

    jsonxf -m <foo.json | gzip -c >foo-min.json.gz

Run `jsonxf -h` to see all configuration options.


## Rust Example

In your `Cargo.toml`:

```
[dependencies]
jsonxf = "1.1"
```

In your code:

```rust
extern crate jsonxf;
let ugly_json = "{\"hello\":\"world\"}";
let pretty_json = jsonxf::pretty_print(ugly_json).unwrap();
assert_eq!(pretty_json, "{\n  \"hello\": \"world\"\n}\n");
```


## Performance

Here are some benchmarks comparing Jsonxf 0.9's performance to
several of its counterparts:
  * [jq](https://stedolan.github.io/jq/), the extremely flexible JSON
    processor.
  * [jsonpp](https://github.com/jmhodges/jsonpp), a JSON pretty-printer
    written in Go.
  * [serdexf](benchmark/serdexf), a trivial example using the
    [serde_json](https://serde.rs/json.html) and
    [serde-transcode](https://serde.rs/transcode.html) libraries.
    This implementation is not complete and is included for library
    comparison only.
  * `cat` is thrown in as well, for scale.

Test platform: MBP (early 2013), macOS 10.13.2, 3GHz i7, 8GB RAM.

See [benchmark.rb](benchmark/benchmark.rb) for testing procedure.

Pretty-print test, 600MB minimized input (1M objects):

| command   | time (s) | relative time | notes |
|-----------|---------:|--------------:|-------|
| `cat`     |     2.26 |          0.3x | `cat` is a bad pretty-printer |
| `jsonxf`  |     7.76 |            1x | |
| `serdexf` |     9.33 |          1.2x | no newlines between objects 🙁 |
| `jsonpp`  |     25.0 |          3.2x | |
| `jq -M .` |     67.0 |          8.6x | |

Minimize test, 850MB pretty-printed input (1M objects):

| command      | time (s) | relative time | notes |
|--------------|---------:|--------------:|-------|
| `cat`        |     1.45 |          0.2x | `cat` is a bad minimizer |
| `jsonxf -m`  |     6.78 |            1x | |
| `serdexf -m` |     7.30 |          1.1x | |
| `jsonpp`     |        - |             - | minimizing is not supported 😭 |
| `jq -cM .`   |     78.0 |           12x | |


## Authorship and License

Copyright 2017-2020, Pete Gamache.

Jsonxf is released under the MIT License.

