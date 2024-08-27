mod arg_parser;

use arg_parser::ArgParser;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek},
};

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
                Ok(file) => buf_reader = io::BufReader::new(file),
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    print_usage(true);
                    return;
                }
            }

            if args.bits & ArgParser::LINES != 0 && args.bits & ArgParser::BYTES != 0 {
                let (lines, bytes) = count_lines_and_bytes(&mut buf_reader);
                println!("{} {} {}", lines, bytes, file_name);
                std::process::exit(0);
            }

            if args.bits & ArgParser::LINES != 0 {
                let len = count_lines(&mut buf_reader);
                print_val(&mut has_printed, len);
                has_printed = true;
                if let Err(err) = buf_reader.rewind() {
                    eprintln!("Error while reading file {}: {}", file_name, err);
                    print_usage(true);
                    return;
                }
            }

            if args.bits & ArgParser::BYTES != 0 {
                let bytes = count_bytes(&mut buf_reader);
                print_val(&mut has_printed, bytes);
                has_printed = true
            }

            if args.bits & ArgParser::WORDS != 0 {
                let words = count_words(&mut buf_reader);
                print_val(&mut has_printed, words);
                has_printed = true;
            }

            if args.bits & ArgParser::CHARS != 0 {
                let chars = count_characters(&mut buf_reader);
                print_val(&mut has_printed, chars);
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
    let mut buf: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut buf).unwrap()
}

fn count_lines_and_bytes(buf_reader: &mut BufReader<File>) -> (usize, usize) {
    let mut lines_total: usize = 0;
    let mut bytes_total: usize = 0;
    let mut str_buf = String::new();
    loop {
        let bytes_count = buf_reader.read_line(&mut str_buf).unwrap();
        if bytes_count == 0 {
            break;
        }
        lines_total += 1;
        bytes_total += bytes_count;
        str_buf.clear();
    }
    (lines_total, bytes_total)
}

fn count_words(buf_reader: &mut BufReader<File>) -> usize {
    let mut words_total: usize = 0;
    let mut str_buf = String::new();
    loop {
        let bytes_count = buf_reader.read_line(&mut str_buf).unwrap();
        if bytes_count == 0 {
            break;
        }
        words_total += str_buf.split_whitespace().count();
        str_buf.clear();
    }
    words_total
}

fn count_characters(buf_reader: &mut BufReader<File>) -> usize {
    let mut characters_count: usize = 0;
    let mut str_buf = String::new();
    while buf_reader.read_to_string(&mut str_buf).unwrap() != 0 {
        characters_count += str_buf.chars().count();
        str_buf.clear();
    }
    characters_count
}