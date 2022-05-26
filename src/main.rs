use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage: {} [TEXT_TO_SEARCH] [FILE]", &args[0])
    }

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}