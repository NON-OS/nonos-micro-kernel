//! Jsonxf is a JSON transformer, supporting fast pretty-printing,
//! minimizing, and similar formatting of JSON-encoded UTF-8 data
//! in string or stream format.  Use it to customize the
//! presentation of valid JSON data.
//!
//! It provides default pretty-printers (`pretty_print()`,
//! `pretty_print_stream()`) and minimizers (`minimize()`,
//! `minimize_stream()`), `Formatter` for customizable JSON
//! formatting, and the `jsonxf` command-line tool (see
//! `jsonxf -h` for options).
//!
//! Jsonxf is built for speed, and does not attempt to perform any
//! input validation whatsoever.  Valid input produces valid output,
//! but no guarantees are offered around the detection and rejection
//! of invalid input.
//!
//! GitHub:
//! <a href="https://github.com/gamache/jsonxf" target="_blank">gamache/jsonxf</a>
//!

use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;
use std::io::ErrorKind;

const BUF_SIZE: usize = 1024 * 16;

const C_CR: u8 = b'\r';
const C_LF: u8 = b'\n';
const C_TAB: u8 = b'\t';
const C_SPACE: u8 = b' ';

const C_COMMA: u8 = b',';
const C_COLON: u8 = b':';
const C_QUOTE: u8 = b'"';
const C_BACKSLASH: u8 = b'\\';

const C_LEFT_BRACE: u8 = b'{';
const C_LEFT_BRACKET: u8 = b'[';
const C_RIGHT_BRACE: u8 = b'}';
const C_RIGHT_BRACKET: u8 = b']';

/// `Formatter` allows customizable pretty-printing, minimizing,
/// and other formatting tasks on JSON-encoded UTF-8 data in
/// string or stream format.
///
/// Example:
///
/// ```
/// let mut fmt = jsonxf::Formatter::pretty_printer();
/// fmt.line_separator = String::from("\r\n");
/// assert_eq!(
///     fmt.format("{\"a\":1}").unwrap(),
///     "{\r\n  \"a\": 1\r\n}"
/// );
/// ```
pub struct Formatter {
    /// Used for beginning-of-line indentation in arrays and objects.
    pub indent: String,

    /// Used inside arrays and objects.
    pub line_separator: String,

    /// Used between root-level arrays and objects.
    pub record_separator: String,

    /// Used after a colon inside objects.
    pub after_colon: String,

    /// Used at very end of output.
    pub trailing_output: String,

    /// Add a record_separator as soon as a record ends, before seeing a
    /// subsequent record. Useful when there's a long time between records.
    pub eager_record_separators: bool,

    // private mutable state
    depth: usize,       // current nesting depth
    in_string: bool,    // is the next byte part of a string?
    in_backslash: bool, // does the next byte follow a backslash in a string?
    empty: bool,        // is the next byte in an empty object or array?
    first: bool,        // is this the first byte of input?
}

impl Formatter {
    fn default() -> Formatter {
        Formatter {
            indent: String::from("  "),
            line_separator: String::from("\n"),
            record_separator: String::from("\n"),
            after_colon: String::from(" "),
            trailing_output: String::from(""),
            eager_record_separators: false,
            depth: 0,
            in_string: false,
            in_backslash: false,
            empty: false,
            first: true,
        }
    }

    /// Returns a Formatter set up for pretty-printing.
    /// Defaults to using two spaces of indentation,
    /// Unix newlines, and no whitespace at EOF.
    ///
    /// # Example:
    ///
    /// ```
    /// assert_eq!(
    ///     jsonxf::Formatter::pretty_printer().format("{\"a\":1}").unwrap(),
    ///     "{\n  \"a\": 1\n}"
    /// );
    /// ```
    pub fn pretty_printer() -> Formatter {
        Formatter::default()
    }

    /// Returns a Formatter set up for minimizing.
    /// Defaults to using Unix newlines between records,
    /// and no whitespace at EOF.
    ///
    /// # Example:
    ///
    /// ```
    /// assert_eq!(
    ///     jsonxf::Formatter::minimizer().format("{  \"a\" : 1  }\n").unwrap(),
    ///     "{\"a\":1}"
    /// );
    /// ```
    pub fn minimizer() -> Formatter {
        let mut xf = Formatter::default();
        xf.indent = String::from("");
        xf.line_separator = String::from("");
        xf.record_separator = String::from("\n");
        xf.after_colon = String::from("");
        xf
    }

    /// Formats a string of JSON-encoded data.
    ///
    /// Input must be valid JSON data in UTF-8 encoding.
    ///
    /// # Example:
    ///
    /// ```
    /// let mut fmt = jsonxf::Formatter::pretty_printer();
    /// fmt.indent = String::from("\t");
    /// fmt.trailing_output = String::from("\n");
    /// assert_eq!(
    ///     fmt.format("{\"a\":1}").unwrap(),
    ///     "{\n\t\"a\": 1\n}\n"
    /// );
    /// ```
    pub fn format(&mut self, json_string: &str) -> Result<String, String> {
        let mut input = json_string.as_bytes();
        let mut output: Vec<u8> = vec![];
        match self.format_stream(&mut input, &mut output) {
            Ok(_) => {}
            Err(f) => {
                return Err(f.to_string());
            }
        };
        let output_string = match String::from_utf8(output) {
            Ok(s) => s,
            Err(f) => {
                return Err(f.to_string());
            }
        };
        Ok(output_string)
    }

    /// Formats a stream of JSON-encoded data.
    ///
    /// Input must be valid JSON data in UTF-8 encoding.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// let mut fmt = jsonxf::Formatter::pretty_printer();
    /// fmt.indent = String::from("\t");
    /// fmt.trailing_output = String::from("\n");
    /// match fmt.format_stream(&mut std::io::stdin(), &mut std::io::stdout()) {
    ///     Ok(_) => { /* YAY */ },
    ///     Err(e) => { panic!(e.to_string()); }
    /// }
    /// ```
    pub fn format_stream(
        &mut self,
        input: &mut dyn Read,
        output: &mut dyn Write,
    ) -> Result<(), Error> {
        let mut reader = BufReader::new(input);
        let mut writer = BufWriter::new(output);
        self.format_stream_unbuffered(&mut reader, &mut writer)
    }

    /// Formats a stream of JSON-encoded data without buffering.
    ///
    /// This will perform many small writes, so it's advisable to use an
    /// output that does its own buffering. In simple cases, use
    /// [`Formatter::format_stream`] instead.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// let mut fmt = jsonxf::Formatter::pretty_printer();
    /// let mut stdin = std::io::stdin();
    /// let mut stdout = std::io::stdout();
    /// fmt.format_stream_unbuffered(&mut stdin, &mut std::io::LineWriter::new(stdout))
    ///     .unwrap();
    /// ```
    pub fn format_stream_unbuffered(
        &mut self,
        input: &mut impl Read,
        output: &mut impl Write,
    ) -> Result<(), Error> {
        let mut buf = [0_u8; BUF_SIZE];
        loop {
            match input.read(&mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    self.format_buf(&buf[0..n], output)?;
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => {
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        output.write_all(self.trailing_output.as_bytes())?;
        Ok(())
    }

    /// Format directly from a buffer into a writer.
    ///
    /// This may be called on chunks of a JSON document to format it bit by bit.
    ///
    /// As such, it does not add the `trailing_output` at the end.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// let text = "[1, 2, 3]";
    /// let mut fmt = jsonxf::Formatter::pretty_printer();
    /// let mut stdout = std::io::stdout();
    /// fmt.format_buf(text.as_bytes(), &mut stdout).unwrap();
    /// ```
    pub fn format_buf(&mut self, buf: &[u8], writer: &mut impl Write) -> Result<(), Error> {
        let mut n = 0;
        while n < buf.len() {
            let b = buf[n];

            if self.in_string {
                if self.in_backslash {
                    writer.write_all(&buf[n..n + 1])?;
                    self.in_backslash = false;
                } else {
                    match memchr::memchr2(C_QUOTE, C_BACKSLASH, &buf[n..]) {
                        None => {
                            // The whole rest of buf is part of the string
                            writer.write_all(&buf[n..])?;
                            break;
                        }
                        Some(index) => {
                            let length = index + 1;
                            writer.write_all(&buf[n..n + length])?;
                            if buf[n + index] == C_QUOTE {
                                // End of string
                                self.in_string = false;
                            } else {
                                // Backslash
                                self.in_backslash = true;
                            }
                            n += length;
                            continue;
                        }
                    }
                }
            } else {
                match b {
                    C_SPACE | C_LF | C_CR | C_TAB => {
                        // skip whitespace
                    }

                    C_LEFT_BRACKET | C_LEFT_BRACE => {
                        if self.first {
                            self.first = false;
                            writer.write_all(&buf[n..n + 1])?;
                        } else if self.empty {
                            writer.write_all(self.line_separator.as_bytes())?;
                            for _ in 0..self.depth {
                                writer.write_all(self.indent.as_bytes())?;
                            }
                            writer.write_all(&buf[n..n + 1])?;
                        } else if !self.eager_record_separators && self.depth == 0 {
                            writer.write_all(self.record_separator.as_bytes())?;
                            writer.write_all(&buf[n..n + 1])?;
                        } else {
                            writer.write_all(&buf[n..n + 1])?;
                        }
                        self.depth += 1;
                        self.empty = true;
                    }

                    C_RIGHT_BRACKET | C_RIGHT_BRACE => {
                        self.depth = self.depth.saturating_sub(1);
                        if self.empty {
                            self.empty = false;
                            writer.write_all(&buf[n..n + 1])?;
                        } else {
                            writer.write_all(self.line_separator.as_bytes())?;
                            for _ in 0..self.depth {
                                writer.write_all(self.indent.as_bytes())?;
                            }
                            writer.write_all(&buf[n..n + 1])?;
                        }
                        if self.eager_record_separators && self.depth == 0 {
                            writer.write_all(self.record_separator.as_bytes())?;
                        }
                    }

                    C_COMMA => {
                        writer.write_all(&buf[n..n + 1])?;
                        writer.write_all(self.line_separator.as_bytes())?;
                        for _ in 0..self.depth {
                            writer.write_all(self.indent.as_bytes())?;
                        }
                    }

                    C_COLON => {
                        writer.write_all(&buf[n..n + 1])?;
                        writer.write_all(self.after_colon.as_bytes())?;
                    }

                    _ => {
                        if self.empty {
                            writer.write_all(self.line_separator.as_bytes())?;
                            for _ in 0..self.depth {
                                writer.write_all(self.indent.as_bytes())?;
                            }
                            self.empty = false;
                        }
                        if b == C_QUOTE {
                            self.in_string = true;
                        }
                        writer.write_all(&buf[n..n + 1])?;
                    }
                };
            };
            n += 1;
        }

        Ok(())
    }
}

/// Pretty-prints a string of JSON-encoded data.
///
/// Input must be valid JSON data in UTF-8 encoding.
///
/// The output will use two spaces as an indent, a line feed
/// as newline character, and no trailing whitespace.
/// To customize this behavior, use a
/// `jsonxf::Formatter::pretty_printer()` directly.
///
/// # Examples:
///
/// ```
/// assert_eq!(
///     jsonxf::pretty_print("{\"a\":1,\"b\":2}").unwrap(),
///     "{\n  \"a\": 1,\n  \"b\": 2\n}"
/// );
/// assert_eq!(
///     jsonxf::pretty_print("{\"empty\":{},\n\n\n\n\n\"one\":[1]}").unwrap(),
///     "{\n  \"empty\": {},\n  \"one\": [\n    1\n  ]\n}"
/// );
/// ```
///
pub fn pretty_print(json_string: &str) -> Result<String, String> {
    Formatter::pretty_printer().format(json_string)
}

/// Pretty-prints a stream of JSON-encoded data.
///
/// Input must be valid JSON data in UTF-8 encoding.
///
/// The output will use two spaces as an indent, a line feed
/// as newline character, and no trailing whitespace.
/// To customize this behavior, use a
/// `jsonxf::Formatter::pretty_printer()` directly.
///
/// `pretty_print_stream` uses `std::io::BufReader` and `std::io:BufWriter`
/// to provide IO buffering; no external buffering should be necessary.
///
/// # Example:
///
/// ```no_run
/// match jsonxf::pretty_print_stream(&mut std::io::stdin(), &mut std::io::stdout()) {
///     Ok(_) => { /* YAY */ },
///     Err(e) => { panic!(e.to_string()) }
/// };
/// ```
///
pub fn pretty_print_stream(input: &mut dyn Read, output: &mut dyn Write) -> Result<(), Error> {
    Formatter::pretty_printer().format_stream(input, output)
}

/// Minimizes a string of JSON-encoded data.
///
/// Input must be valid JSON data in UTF-8 encoding.
///
/// The output will use a line feed as newline character between
/// records, and no trailing whitespace.  To customize this behavior,
/// use a `jsonxf::Formatter::minimizer()` directly.
///
/// # Examples:
///
/// ```
/// assert_eq!(
///     jsonxf::minimize("{ \"a\": \"b\", \"c\": 0 } ").unwrap(),
///     "{\"a\":\"b\",\"c\":0}"
/// );
/// assert_eq!(
///     jsonxf::minimize("\r\n\tnull\r\n").unwrap(),
///     "null"
/// );
/// ```
///
pub fn minimize(json_string: &str) -> Result<String, String> {
    Formatter::minimizer().format(json_string)
}

/// Minimizes a stream of JSON-encoded data.
///
/// Input must be valid JSON data in UTF-8 encoding.
///
/// The output will use a line feed as newline character between
/// records, and no trailing whitespace.  To customize this behavior,
/// use a `jsonxf::Formatter::minimizer()` directly.
///
/// `minimize_stream` uses `std::io::BufReader` and `std::io:BufWriter`
/// to provide IO buffering; no external buffering should be necessary.
///
/// # Example:
///
/// ```no_run
/// match jsonxf::minimize_stream(&mut std::io::stdin(), &mut std::io::stdout()) {
///     Ok(_) => { /* YAY */ },
///     Err(e) => { panic!(e.to_string()) }
/// };
/// ```
///
pub fn minimize_stream(input: &mut dyn Read, output: &mut dyn Write) -> Result<(), Error> {
    Formatter::minimizer().format_stream(input, output)
}
