pub mod key_parser {

    use std::path::Path;
    use std::ffi::OsStr;
    use lib::config::Config;
    use lib::crypt::Algo;
    use lib::crypt::Key;
    use lib::crypt::parse_hex;
    use std::fs;

    fn get_extension(filename: &str) -> Option<&str> {
        Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
    }

    fn parse_key(filename: &str, algo: &Algo) -> Result<Key, String> {
        let content = fs::read_to_string(format!("{}.key", &filename))
            .map_err(|e| e.to_string())?
            .trim()
            .to_string();
        match algo {
            Algo::Xor => parse_hex(&content).map(Key::Xor),
            Algo::Aes => todo!(),
        }
    }

pub fn resolve_encryption(file: &str, algo_override: Option<Algo>,
    key_override: Option<String>, config: &Config) -> Result<(Algo, Key), String> {
    
    let used_algo = match algo_override {
        Some(algo) => algo,
        None => {
            let ext = get_extension(file).ok_or("No file extension found")?;
            match ext {
                "xor" => Algo::Xor,
                "aes" => Algo::Aes,
                _ => return Err(format!("Unknown extension: {}", ext))
            }
        }
    };
    let used_key = match key_override {
        Some(k) => match &used_algo {
            Algo::Xor => Key::Xor(parse_hex(&k)?),
            Algo::Aes => todo!(),
        },
        None => parse_key(file, &used_algo)?
    };
    Ok((used_algo, used_key))
}
}