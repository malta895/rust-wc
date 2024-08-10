use std::{env, io::{self, BufRead, Read},};

use rust_wc::ArgParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    if args.len() < 2 {
        print_usage(true);
    }
    let result = ArgParser::from_args(args.to_vec());

    match result {
        Ok(args) => {
            let file_name = args.file_name;
            let mut has_printed: bool = false;

            if args.bits & ArgParser::LINES != 0 {
                let lines = count_lines(&file_name).unwrap();
                print_val(&mut has_printed, lines);
            }

            if args.bits & ArgParser::BYTES != 0 {
                let bytes = count_bytes(&file_name).unwrap();
                print_val(&mut has_printed, bytes);
            }

            print!(" {}\n", file_name);
        }
        Err(e) => {
            eprintln!("{}", e);
            print_usage(true);
        }
    }    
}


fn print_val(has_printed: &mut bool, val: usize) {
    if *has_printed {
        print!(" ");
    }
    print!("{}", val);
    *has_printed = true;
}

fn print_usage(with_error: bool) {
    eprintln!("Usage: rust-wc [-c -l -w -m] <file>");
    if with_error {
        std::process::exit(1);
    }
}


fn count_lines(file_name: &str) -> io::Result<usize> {
    let file = std::fs::File::open(file_name)?;
    let buf_reader = io::BufReader::new(file);
    Ok(buf_reader.lines().count())
}

fn count_bytes(file_name: &str) -> io::Result<usize> {
    let file = std::fs::File::open(file_name)?;
    let buf_reader = io::BufReader::new(file);
    Ok(buf_reader.bytes().count())
}

