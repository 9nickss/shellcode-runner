use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 { // avec key custom ou key de base
        eprintln!("Wrong number of arguments");
        std::process::exit(1);
    }
    if args.len() == 3 {
        let key = args[2];
    }
    let key = 0xAA;
    
}