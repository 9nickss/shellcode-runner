pub mod key_parser {

    use std::path::Path;
    use std::ffi::OsStr;
    use lib::config::Config;
    use lib::crypt::Algo;
    use lib::crypt::Key;
    use lib::crypt::parse_hex;
    use std::fs;
    use lib::crypt;

    fn get_extension(filename: &str) -> Option<&str> {
        Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
    }

    fn parse_key(filename: &str, algo: &Algo, config: &Config) -> Result<Key, String> {
        config.log(&format!("No key override, reading {}.key...", filename));
        let content = fs::read_to_string(format!("{}.key", &filename))
            .map_err(|e| e.to_string())?
            .trim()
            .to_string();
        config.log("Key loaded from file");
        match algo {
            Algo::Xor => parse_hex(&content).map(Key::Xor),
            Algo::Aes => crypt::parse_hex_bytes(&content).map(Key::Aes),
        }
    }

pub fn resolve_encryption(file: &str, algo_override: Option<Algo>,
    key_override: Option<String>, config: &Config) -> Result<(Algo, Key), String> {
    
    config.log("Resolving key...");
    let used_algo = match algo_override {
        Some(algo) => {
            config.log(&format!("Using algo override: {:?}", algo));
            algo
        },
        None => {
            let ext = get_extension(file).ok_or("No file extension found")?;
            config.log(&format!("Extension detected: .{}", ext));
            match ext {
                "xor" => Algo::Xor,
                "aes" => Algo::Aes,
                _ => return Err(format!("Unknown extension: {}", ext))
            }
        }
    };
    let used_key = match key_override {
        Some(k) => {
            config.log("Using key override from args...");
            match &used_algo {
                Algo::Xor => Key::Xor(parse_hex(&k)?),
                Algo::Aes => crypt::parse_hex_bytes(&k  ).map(Key::Aes)?,
            }
        },
        None => parse_key(file, &used_algo, &config)?
    };
    match &used_key {
        Key::Xor(k) => config.log(&format!("Key resolved: 0x{:02X}", k)),
        Key::Aes(k) => config.log(&format!("Key resolved: AES-128 [{:02X}{:02X}...{:02X}]", k[0], k[1], k[15])),
    }
    Ok((used_algo, used_key))
}
}
