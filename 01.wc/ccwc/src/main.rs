mod counts;

use std::env;
use std::io::{self, Read};
#[allow(unused_imports)]


fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();

    match args.len(){
        1 => {
            let mut input = String::new();
            stdin.lock().read_to_string(&mut input).unwrap();
            print_counts(&input);
        },
        3=>{
            let option = &args[1];
            let file_path = &args[2];
            match option.as_str() {
                "-c" => println!("Bytes: {}", counts::count_bytes(file_path).unwrap()),
                "-w" => println!("Words: {}", counts::count_words(file_path).unwrap()),
                "-l" => println!("Lines: {}", counts::count_lines(file_path).unwrap()),
                "-m" => println!("Characters: {}", counts::count_characters(file_path).unwrap()),
                _ => println!("Invalid option")
            }
        },
        _ => println!("Usage: ccwc [-c | -w | -l | -m] [file_path]")
    }
}

fn print_counts(input: &str){
    println!("Bytes: {}", input.as_bytes().len());
    println!("Words: {}", input.split_whitespace().count());
    println!("Lines: {}", input.lines().count());
    println!("Characters: {}", input.chars().count());
}
