/*
  Jsonxf: a fast JSON pretty-printer and minimizer.
   ,
  (k) 2017-2018 pete gamache <pete@gamache.org>

  Input must be valid JSON-encoded data in UTF-8 format.

  Run `jsonxf -h` for usage options.
*/

use std::{fs::File, io::ErrorKind};

extern crate jsonxf;

extern crate getopts;
use getopts::Options;

fn main() {
    match do_main() {
        Ok(_) => { /* YAY */ }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
}

fn do_main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    opts.optopt(
        "s",
        "string",
        "use the given string as input, instead of a file",
        "str",
    );
    opts.optopt(
        "i",
        "input",
        "read input from the given file (default: stdin)",
        "file",
    );
    opts.optopt(
        "o",
        "output",
        "write output to the given file (default: stdout)",
        "file",
    );
    opts.optopt(
        "t",
        "tab",
        "use the given string to indent pretty-printed output (default: two spaces)",
        "tabstr",
    );
    opts.optflag(
        "m",
        "minimize",
        "minimize JSON instead of pretty-printing it",
    );
    opts.optflag("h", "help", "print this message and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    if matches.opt_present("h") {
        let program = args[0].clone();
        print_help(&program, &opts);
        return Ok(());
    }

    // If these are set and match later, we need to take care not to
    // truncate the input file.
    let mut input_filename: Option<String> = None;
    let mut output_filename: Option<String> = None;
    let mut output_temp_filename: Option<String> = None;

    let mut input_str = String::from("");
    let mut input: Box<dyn std::io::Read> = match matches.opt_str("i") {
        None => match matches.opt_str("s") {
            None => Box::new(std::io::stdin()),
            Some(json_str) => {
                input_str.push_str(&json_str);
                Box::new(input_str.as_bytes())
            }
        },
        Some(filename) => {
            if filename == *"-" {
                Box::new(std::io::stdin())
            } else {
                input_filename = Some(String::from(&filename));
                match File::open(&filename) {
                    Ok(f) => Box::new(f),
                    Err(e) => {
                        let mut estr = filename;
                        estr.push_str(": ");
                        estr.push_str(&e.to_string());
                        return Err(estr);
                    }
                }
            }
        }
    };

    let mut output: Box<dyn std::io::Write> = match matches.opt_str("o") {
        None => Box::new(std::io::stdout()),
        Some(filename) => {
            if filename == *"-" {
                Box::new(std::io::stdout())
            } else {
                let mut temp_filename = String::from(&filename);

                match input_filename {
                    None => (),
                    Some(f) => {
                        if f == filename {
                            temp_filename.push_str(".tmp");
                        }
                    }
                }

                output_filename = Some(String::from(&filename));
                output_temp_filename = Some(String::from(&temp_filename));

                match File::create(&temp_filename) {
                    Ok(f) => Box::new(f),
                    Err(e) => {
                        let mut estr = filename;
                        estr.push_str(": ");
                        estr.push_str(&e.to_string());
                        return Err(estr);
                    }
                }
            }
        }
    };

    let indent = match matches.opt_str("t") {
        None => String::from("  "),
        Some(string) => string,
    };

    let result = if matches.opt_present("m") {
        let mut xf = jsonxf::Formatter::minimizer();
        xf.format_stream(&mut input, &mut output)
    } else {
        let mut xf = jsonxf::Formatter::pretty_printer();
        xf.indent = indent;
        // Ensure a trailing newline, as expected on Unix
        xf.eager_record_separators = true;
        xf.format_stream(&mut input, &mut output)
    };

    match output_temp_filename {
        None => (),
        Some(temp_filename) => std::fs::rename(temp_filename, output_filename.unwrap()).unwrap(),
    }

    match result {
        Err(e) if e.kind() == ErrorKind::BrokenPipe => Ok(()),
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(()),
    }
}

fn print_help(program_name: &str, opts: &Options) {
    let desc = "Jsonxf is a JSON transformer.  It provides fast pretty-printing and
minimizing of JSON-encoded UTF-8 data.";

    let examples = "
Pretty-print a string to the terminal, using two spaces to indent:

    jsonxf -s '{\"a\": {\"b\": 2, \"c\": false}}'

Pretty-print and read a JSON file, using a tab character to indent:

    jsonxf -t $'\\t' <foo.json | less

Minimize a file and gzip it:

    jsonxf -m <foo.json | gzip -c >foo-min.json.gz
";

    let brief = format!("Usage: {} [options]\n\n{}", program_name, desc);
    print!("{}", opts.usage(&brief));
    println!("{}", examples);
}
