// Measurement harness for the paper's evaluation. Prints real trace shapes and
// NOX fees for a handful of programs, produced by running the crate.
use nonos_zkolang::{prove_source_with_inputs, quote};

fn main() {
    let cases: [(&str, &str, &[u64]); 4] = [
        (
            "demo (add,mul,assert,output)",
            "let a=3; let b=5; let s=a+b; let p=s*s; assert p-64; output p;",
            &[],
        ),
        ("square  y=x^2", "input x; let y=x*x; output y;", &[9]),
        ("cube    y=x^3", "input x; let y=x*x*x; output y;", &[3]),
        ("degree8 y=x^8", "input x; let a=x*x; let b=a*a; let c=b*b; output c;", &[2]),
    ];
    for (name, src, inputs) in cases {
        let r = prove_source_with_inputs(src, inputs).expect("run");
        let q = quote(&r);
        println!(
            "{name:32} steps={:2} trace=2^{} x {} = {:5} cells  verified={}  outputs={:?}  fee={} uNOX (prover {}, protocol {})",
            r.steps, r.log_trace_len, r.trace_width, r.trace_len * r.trace_width,
            r.verified, r.outputs, q.total_micronox, q.prover_micronox, q.protocol_fee_micronox
        );
    }
}
