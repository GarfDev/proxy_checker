use std::fs::File;
use std::io::{stdin,stdout,Write,BufReader};
use std::io::prelude::*;
use std::path::Path;

fn read_and_add(path: &str, mut proxy_list: Vec<String>) -> Vec<String> {
    // Create a path to the desired file
    let path = Path::new(path);
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {:?}", display, why.kind()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                proxy_list.insert(proxy_list.len(), line);
                
            }
            Err(e) => println!("ERROR: {}", e),
        }
    }

    proxy_list
}

fn main() {
    // Initial_variable
    let mut input = String::new();
    print!("Please enter path: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    let proxy_list = vec![];
    println!("{:?}", input.as_ptr());
    // proxy_list_moved_to_here
    let proxy_list = read_and_add(&input[0..input.len() - 1].to_string() ,proxy_list);
    // proxy_list_return_to_here
    println!("{}", proxy_list.len());
}