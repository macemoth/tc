use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use tiktoken_rs::cl100k_base;

fn read_file_with_encoding(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).into_owned())
}

fn main() {
    let mut input = String::new();
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        // Read from file(s) specified as arguments
        for filename in &args[1..] {
            match read_file_with_encoding(Path::new(filename)) {
                Ok(file_content) => input.push_str(&file_content),
                Err(e) => eprintln!("Error reading file {}: {}", filename, e),
            }
        }
    } else {
        // If no arguments, try to read from stdin
        io::stdin().read_to_string(&mut input).unwrap();
    }
    
    let encoding = cl100k_base().unwrap();
    let tokens = encoding.encode_with_special_tokens(&input);
    
    println!("{}", tokens.len());
}
