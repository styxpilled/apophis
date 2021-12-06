use std::{env, process};

mod apophis;

const USAGE: &str = "
apophis - Kill processes by name

USAGE:
    apophis [options] <process name>

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints version information
    -V, --verbose    Prints verbose information
";

fn main() {
    match env::args().nth(1) {
        None => {
            apophis::run(false)
        }
        Some(arg) => {
            match arg.as_str() {
                "-h" | "--help" => {
                    println!("{}", USAGE)
                }
                "-v" | "--version" => {
                    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                }
                "-V" | "--verbose" => {
                    apophis::run(true)
                }
                _ => {
                    println!("Unknown argument: {}", arg);
                    process::exit(1);
                }
            }
        }
    }
}
