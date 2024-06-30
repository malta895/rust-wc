use std::{env, io::{self, Read}};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: rust-wc -c <file>");
        std::process::exit(1);
    }
    let file_name = &args[2].as_str();
    match count_bytes(file_name){
        Ok(count) => println!("{} {}", count, file_name),
        Err(e) =>{
            eprintln!("Error: {}", e);
            std::process::exit(1);
        },
    }
}


fn count_bytes(file_name: &str) -> io::Result<usize> {
    println!("file_name: {}", file_name);
    match std::fs::File::open(file_name) {
        Ok(file) => Ok(file.bytes().count()),
        Err(e) => Err(e),
    }
}

