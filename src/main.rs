use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek},
};

mod arg_parser;
use arg_parser::ArgParser;

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
            let mut buf_reader: BufReader<File>;

            match std::fs::File::open(file_name.clone()) {
                Ok(file) => {
                    buf_reader = io::BufReader::new(file);
                }
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    print_usage(true);
                    return;
                }
            }

            if args.bits & ArgParser::LINES != 0 {
                let len = count_lines(&mut buf_reader);
                print_val(&mut has_printed, len);
                has_printed = true;
                if let Err(err) = buf_reader.seek(io::SeekFrom::Start(0)){
                    eprintln!("Error while reading file {}: {}", file_name, err);
                    print_usage(true);
                    return
                }
            }

            if args.bits & ArgParser::BYTES != 0 {
                let bytes = count_bytes(&mut buf_reader);
                print_val(&mut has_printed, bytes);
                has_printed = true
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

fn count_lines(buf_reader: &mut BufReader<File>) -> usize {
    buf_reader.lines().count()
}

fn count_bytes(buf_reader: &mut BufReader<File>) -> usize {
    buf_reader.bytes().count()
}
