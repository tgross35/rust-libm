use std::path::Path;

mod serialized_tests;
use std::env;
mod traits;

const USAGE: &str = "\
usage:

cargo run -p util -- <SUBCOMMAND>

SUBCOMMAND:
    generate-tests <OUTDIR>:    write output to files located in OUTDIR
";

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let args_str = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    match args_str.as_slice()[1..] {
        ["generate-tests", outdir] => create_serialized_tests(outdir),
        _ => {
            println!("{USAGE}, `{args_str:?}`");
            std::process::exit(1);
        }
    }
}

fn create_serialized_tests(outdir: &str) {
    serialized_tests::create(Path::new(outdir));
}
