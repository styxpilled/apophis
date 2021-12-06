// use sysinfo::{ProcessExt, System, SystemExt, Signal};
use sysinfo::{ProcessExt, System, SystemExt};
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn run(verbose: bool) {
    let mut names = vec![];
    if let Ok(lines) = read_lines("./.apophis") {
        for line in lines {
            if let Ok(ip) = line {
                names.push(ip.to_lowercase());
            }
        }
    }

    let mut sys = System::new_all();
    sys.refresh_all();
    let mut ret = vec![];
    for (_pid, process) in sys.processes() {
        for name in &names {
            if process.name().to_lowercase().contains(name) {
                ret.push(process.name());
            }
        }
    }
    for process in ret {
        if verbose {
            println!("Killed: {}", process);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}