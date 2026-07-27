# Changelog

## 1.1.1 (2021-04-13)

Cleaned up some syntax. Thanks, `cargo clippy`!

## 1.1.0 (2021-04-09)

Added a huge pile of improvements documented in [the Github
PR](https://github.com/gamache/jsonxf/pull/8). Thanks again,
[@blyxxyz](https://github.com/blyxxyz)!

## 1.0.2 (2021-02-09)

Fixed bug where an input of an unmatched closing bracket would
result in a surprisingly deep indent. Thanks,
[@blyxxyz](https://github.com/blyxxyz)!

## 1.0.1 (2020-11-03)

Fixed bug where `jsonxf -i foo -o foo` would truncate `foo`.
Thanks for the report, [@anthhub](https://github.com/anthhub)!

## 1.0.0 (2020-06-29)

Updated to latest `getopts` and fixed some deprecated style.
Thanks, [@veer66](https://github.com/veer66)!

## 0.9.0 (2018-01-15)

Breaking change: removed `indent` parameter from `pretty_print()` and
`pretty_print_stream()`.

Added `Formatter` to support customized formatting.

Major refactor.

## 0.7.0 (2018-01-01)

Performance improvements and an improved benchmark harness.

## 0.6.0 (2017-12-31)

Added `-s` command-line option to transform a JSON string.

## 0.5.0 (2017-12-29)

First release: command-line pretty-printing and minimization of
files and streams, Rust pretty-printing and minimization of streams
and strings.

