use std::io;

pub struct ArgParser {
    pub bits: u32,
    pub file_name: String,
}

impl ArgParser {
    pub const BYTES: u32 = 0b0001;
    pub const LINES: u32 = 0b0010;
    pub const WORDS: u32 = 0b0100;
    pub const CHARS: u32 = 0b1000;
    pub const ALL: u32 = 0b1111;

    pub fn from_args(args: Vec<String>) -> io::Result<Self> {
        let mut bits: u32 = 0;
        let mut file_name: String = String::new();
        for (_, value) in args.iter().enumerate() {
            match value.as_str() {
                "-c" => bits |= Self::BYTES,
                "-l" => bits |= Self::LINES,
                "-w" => bits |= Self::WORDS,
                "-m" => bits |= Self::CHARS,
                cmd_arg => {
                    if cmd_arg.starts_with("-") {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("Invalid flag provided {}", cmd_arg),
                        ));
                    }
                    if file_name != "" {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("Multiple files specified"),
                        ));
                    }
                    file_name = value.clone();
                }
            }
        }
        if bits == 0 {
            bits = Self::ALL;
        }
        Ok(Self { bits, file_name })
    }
}

#[cfg(test)]
mod tests {
    use crate::arg_parser::ArgParser;

    #[test]
    fn should_create_from_args_with_c() {
        let result = ArgParser::from_args(vec!["-c".to_string(), "file.txt".to_string()]).unwrap();
        assert_eq!(result.bits, ArgParser::BYTES);
        assert_eq!(result.file_name, "file.txt")
    }

    #[test]
    fn should_create_from_args_with_l() {
        let result = ArgParser::from_args(vec!["-l".to_string(), "file.txt".to_string()]).unwrap();
        assert_eq!(result.bits, ArgParser::LINES);
    }

    #[test]
    fn should_create_from_args_with_l_and_c() {
        let result = ArgParser::from_args(vec![
            "-l".to_string(),
            "-c".to_string(),
            "somefile.txt".to_string(),
        ])
        .unwrap();

        assert_eq!(result.bits, ArgParser::LINES | ArgParser::BYTES);
    }

    #[test]
    fn should_create_from_args_with_all_flags() {
        let result = ArgParser::from_args(vec![
            "-l".to_string(),
            "-c".to_string(),
            "-w".to_string(),
            "-m".to_string(),
            "somefile.txt".to_string(),
        ])
        .unwrap();

        assert_eq!(result.bits, ArgParser::ALL);
    }

    #[test]
    fn should_create_from_args_with_no_flags() {
        let result = ArgParser::from_args(vec!["somefile.txt".to_string()]).unwrap();

        assert_eq!(result.bits, ArgParser::ALL);
    }
}
