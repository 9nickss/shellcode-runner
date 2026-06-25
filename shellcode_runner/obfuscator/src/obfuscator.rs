use lib::obfuscation::inject_junk;
use lib::config::Config;

fn main() {
    let config = Config::new(true); // verbose hardcodé pour tester
    let mut shellcode = std::fs::read("../shellcodes/write.bin")
        .expect("shellcode introuvable");
    inject_junk(&mut shellcode, 0.3, &config).unwrap();
}