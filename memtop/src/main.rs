use memtop::*;
use std::env;
use std::path::Path;

fn main() {
    let mut is_dry_run = false;
    let mut expect_proc_dir = false;
    let mut proc_dir = String::from("/proc/");

    for argument in env::args().skip(1) {
        if expect_proc_dir {
            expect_proc_dir = false;
            proc_dir = argument;
        } else if argument == "--dry-run" {
            is_dry_run = true;
        } else if argument == "--proc-root" {
            expect_proc_dir = true;
        } else {
            std::process::exit(2);
        }
    }

    if !is_dry_run || expect_proc_dir {
        std::process::exit(2);
    }
    let mem_info_file = Path::new(&proc_dir).join(MEMINFO_FILE);
    let contents = match std::fs::read_to_string(&mem_info_file) {
        Ok(text) => text,
        Err(_e) => std::process::exit(1),
    };

    if let Some(mem_info) = parse_meminfo(&contents) {
        println!("MEMTOP_SUMMARY|{}", mem_info);
    } else {
        std::process::exit(1);
    }
}
