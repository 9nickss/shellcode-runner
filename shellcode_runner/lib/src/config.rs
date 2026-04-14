pub struct Config {
    pub verbose: bool,
    pub key: Option<u8>,
}

impl Config {
    pub fn new(verbose: bool, key: Option<u8>) -> Self {
        Config { verbose, key }
    }
    
    pub fn log(&self, msg: &str) {
        if self.verbose {
            println!("[*] {}", msg);
        }
    }
}
