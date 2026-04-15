pub struct Config {
    pub verbose: bool,
}

impl Config {
    pub fn new(verbose: bool, key: Option<u8>) -> Self {
        Config { verbose }
    }
    
    pub fn log(&self, msg: &str) {
        if self.verbose {
            println!("[*] {}", msg);
        }
    }
}
