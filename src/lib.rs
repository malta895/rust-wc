use std::{fs::File, io, };

pub struct ArgParser {
    bits: u32,
    file: File,
}

impl ArgParser {
    const BYTES: u32= 0b0001;
    const LINES: u32= 0b0010;
    const WORDS: u32= 0b0100;
    const CHARS: u32= 0b1000;
    const ALL: u32= 0b1111;

    pub fn from_args(args: Vec<String>) -> io::Result<Self>{
        let mut bits: u32 = 0;
        let mut file: Option<File> = None;
        for (_, value) in args.iter().enumerate() {
            if !value.starts_with("-") {
                if file.is_some() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Only one file is allowed"));
                }
                file = match File::open(&value) {
                    Ok(file) => Some(file),
                    Err(e) => return Err(
                        io::Error::new(io::ErrorKind::InvalidInput, 
                            format!("Error with file at {}: error {}", &value, e),
                        )
                    ),
                };
                continue;
            }
            match value.as_str() {
                "-c" => bits |= Self::BYTES,
                "-l" => bits |= Self::LINES,
                "-w" => bits |= Self::WORDS,
                "-m" => bits |= Self::CHARS,
                invalid_flag => return Err(io::Error::new(
                    io::ErrorKind::InvalidInput, 
                    format!(  "Invalid flag provided {}", invalid_flag) 
                )),
            }
        }
        if bits == 0 {
            bits = Self::ALL;
        }
        match file {
            Some(file) => Ok(ArgParser{bits, file}),
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "No file provided")),
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::ArgParser;
    use std::env;
    use std::io::Read;
    use std::path::PathBuf;

    
    fn setup_temp_file(name: &str) -> PathBuf {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(name);
        std::fs::write(&temp_dir, "Hello, World!").unwrap();        
        temp_dir
    }

    #[test]
    fn should_create_from_args_with_c() {
        let temp_dir = setup_temp_file("file1.txt");
        let path = temp_dir.as_path();
        let path_string = path.to_str().unwrap().to_string();
        let mut result = ArgParser::from_args(
            vec!["-c".to_string(), path_string.clone()]
        )
            .unwrap();
        assert_eq!(result.bits, ArgParser::BYTES);
        let mut buf : Vec<u8> = Vec::new();
        let file_res = result.file.read_to_end(&mut buf);
        assert_eq!(file_res.unwrap(), 13);
        assert_eq!(buf, b"Hello, World!");
    }

    #[test]
    fn should_create_from_args_with_l() {
        let temp_dir = setup_temp_file("file2.txt");
        let path = temp_dir.as_path();
        let path_string = path.to_str().unwrap().to_string();
        let mut result = ArgParser::from_args(
            vec!["-l".to_string(), path_string.clone()]
        )
            .unwrap();
        assert_eq!(result.bits, ArgParser::LINES);
        let mut buf : Vec<u8> = Vec::new();
        let file_res = result.file.read_to_end(&mut buf);
        assert_eq!(file_res.unwrap(), 13);
        assert_eq!(buf, b"Hello, World!");
    }

    #[test]
    fn should_create_from_args_with_l_and_c() {
        let temp_dir = setup_temp_file("file3.txt");
        let path = temp_dir.as_path();
        let path_string = path.to_str().unwrap().to_string();
        let mut result = ArgParser
        ::from_args(
            vec![
                "-l".to_string(),
                "-c".to_string(),
                 path_string.clone()
                 ]
        )
            .unwrap();

        assert_eq!(result.bits, ArgParser::LINES | ArgParser::BYTES);
        let mut buf : Vec<u8> = Vec::new();
        let file_res = result.file.read_to_end(&mut buf);
        assert_eq!(file_res.unwrap(), 13);
        assert_eq!(buf, b"Hello, World!");
    }

    #[test]
    fn should_create_from_args_with_all_flags() {
        let temp_dir = setup_temp_file("file4.txt");
        let path = temp_dir.as_path();
        let path_string = path.to_str().unwrap().to_string();
        let mut result = ArgParser
        ::from_args(
            vec![
                "-l".to_string(),
                "-c".to_string(),
                "-w".to_string(),
                "-m".to_string(),
                 path_string.clone()
                 ]
        )
            .unwrap();

        assert_eq!(result.bits, ArgParser::ALL);
        let mut buf : Vec<u8> = Vec::new();
        let file_res = result.file.read_to_end(&mut buf);
        assert_eq!(file_res.unwrap(), 13);
        assert_eq!(buf, b"Hello, World!");
    }

    #[test]
    fn should_create_from_args_with_no_flags() {
        let temp_dir = setup_temp_file("file4.txt");
        let path = temp_dir.as_path();
        let path_string = path.to_str().unwrap().to_string();
        let mut result = ArgParser
        ::from_args(
            vec![
                 path_string.clone()
                 ]
        )
            .unwrap();

        assert_eq!(result.bits, ArgParser::ALL);
        let mut buf : Vec<u8> = Vec::new();
        let file_res = result.file.read_to_end(&mut buf);
        assert_eq!(file_res.unwrap(), 13);
        assert_eq!(buf, b"Hello, World!");
    }
}