use std::fs::File;
use std::thread;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader, Write};
use std::sync::{Arc, Mutex};

use std::path::Path;
use std::time::Duration;

fn read_and_add(path: &str, mut proxy_list: Vec<String>) -> Vec<String> {
    // Create a path to the desired file
    let path = Path::new(path);
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {:?}", display, why.kind()),
        Ok(file) => file,
    };
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        match line {
            Ok(line) => {
                proxy_list.insert(proxy_list.len(), line);
            }
            Err(error) => println!("ERROR: {:?}", error.kind()),
        }
    }

    proxy_list
}

// fn threads_operating(proxy_list: Vec<String>) {
//     // nothing for now
// }

// async fn check_proxy(proxy: String) {
//     let proxy_string = format!("http://{}", proxy);
//     match reqwest::Proxy::all(&proxy_string) {
//         Ok(proxy) => {
//             match reqwest::Client::builder().timeout(Duration::from_secs(3))
//             .danger_accept_invalid_certs(true).proxy(proxy).build() {
//                 Ok(client) => {
//                     match client.get("https://api.ipify.org?format=json").send().await {
//                         Ok(response) => {
//                             println!("{:?}", response)
//                         }
//                         Err(error) => {
//                             println!("Error: {:?}", error)
//                         }
//                     }
//                 }
//                 Err(error) => {
//                     println!("Error: {}", error);
//                 }      
//             }
            
//         }
//         Err(error) => println!("Error: {}", error),
//     }
// }



fn main() {
    // Initial_variable

    let mut input = String::new();
    print!("Please enter path: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    let proxy_list = vec![];
    println!("{:?}", input.as_ptr());
    let proxy_list = read_and_add(&input[0..input.len() - 1].to_string() ,proxy_list); // <--- proxy_list moved to here and returned by the function
    let proxy_list = Arc::from(Mutex::from(proxy_list));
    let mut child_threads = vec![];

    for _ in 0..100 {
        let proxy_data = Arc::clone(&proxy_list);
        child_threads.push(thread::spawn(move || {
            match proxy_data.lock() {
                Ok(data) => {
                    let mut data = data;
                    if data.len() > 0 {
                        let proxy = data.pop();
                        let proxy_string = format!("http://{:?}", proxy);
                        let client = reqwest::Client::builder().timeout(Duration::from_secs(3)).danger_accept_invalid_certs(true).proxy(reqwest::Proxy::all(&proxy_string).unwrap()).build().unwrap();
                        println!("{:?}", client);
                        // let _resp = client.get("https://api.ipify.org?format=json").send();      
                    }
                }
                Err(error) => println!("{}", error)
            }
        }).join()
    )
    }

    println!("{:?}", child_threads);

}
