use std::io::Read;

use musicxml;

fn load_file(path: &str) -> String {
    let mut file = std::fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\tcargo run --example read -- input.xml");
        std::process::exit(1);
    }

    let text = load_file(&args[1]);
    let mxml = musicxml::Score::from(text);

    println!("{:?}", mxml);
}
