// Allow things that I think is good for my code style

#![allow(non_camel_case_types)]

// Import libraries, I known this useless but this is for splitting parts of code.

extern crate reqwest;
extern crate tokio;

use colored::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader, Write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;

mod constants;
mod runner;

// Functions

// fn read_combo_file(path: &str, mut combo_list: Vec<constants::Combo>) -> Vec<constants::Combo> {
//     // Create a path to the desired file
//     let path = Path::new(path);
//     let display = path.display();
//     // Open the path in read-only mode, returns `io::Result<File>`
//     let file = match File::open(&path) {
//         Err(why) => panic!("Couldn't open {}: {:?}", display, why.kind()),
//         Ok(file) => file,
//     };
//     let buffer = BufReader::new(file);
//     for line in buffer.lines() {
//         match line {
//             Ok(line) => {
//                 let spliced_line: Vec<&str> = line.split(",").collect();
//                 let combo = constants::Combo {
//                     email: String::from(spliced_line[0]),
//                     password: String::from(spliced_line[1]),
//                 };

//                 combo_list.insert(combo_list.len(), combo);
//             }
//             Err(_error) => {
//                 // println!("ERROR: {:?}", error.kind())
//             }
//         }
//     }

//     combo_list
// }

fn read_proxy_file(path: &str, mut proxy_list: Vec<String>) -> Vec<String> {
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
            Err(_error) => {
                // println!("ERROR: {:?}", error.kind())
            }
        }
    }

    proxy_list
}

// Mainstream

fn main() {
    // Showing Flexing things
    println!("{}", constants::LOGO.magenta().bold());
    println!("{}", constants::TEXTLINE.bold());

    // Progress Proxy Path and File
    let mut proxy_path = String::new();
    print!("Proxy path: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut proxy_path)
        .expect("Did not enter a correct string");
    let proxy_list = vec![];
    let mut proxy_list =
        read_proxy_file(&proxy_path[0..proxy_path.len() - 2].to_string(), proxy_list); // <--- proxy_list moved to here and returned by the function

    // Progress Combo Path and File
    // let mut combo_path = String::new();
    // print!("Combo path: ");
    // let _ = stdout().flush();
    // stdin()
    //     .read_line(&mut combo_path)
    //     .expect("Did not enter a correct string");
    // let combo_list = vec![];
    // let mut combo_list =
    //     read_combo_file(&combo_path[0..combo_path.len() - 2].to_string(), combo_list); // <--- proxy_list moved to here and returned by the function

    // Initialize Important things
    let mut child_threads = vec![];
    let mut terminated_threads = 0;
    let (sender, receiver) = channel::<constants::Action>();

    for i in 0..200 {
        let sender = sender.clone();
        let thread = thread::spawn(move || {
            let (tx, rx) = channel::<constants::Action>();

            loop {
                thread::sleep(Duration::from_secs(1));
                let tx = tx.clone();
                let response = constants::Action {
                    kind: constants::ActionTypes::REQUIRE_JOB,
                    payload: format!("Send from thread {}", i).to_string(),
                    sender: Some(tx),
                };
                let _ = sender.send(response);
                match rx.recv() {
                    Ok(response) => {
                        match response.kind {
                            // Unhandled Actions
                            constants::ActionTypes::REQUIRE_JOB => {}
                            constants::ActionTypes::JOB_SUCCESS => {}
                            constants::ActionTypes::JOB_FAILED => {}
                            // Handle Actions
                            constants::ActionTypes::JOB_REQUIRE_APPROVED => {
                                let proxy_string = format!("http://{}", response.payload);
                                let resp = runner::check_proxy(proxy_string);
                                match resp.success {
                                    true => {
                                        let response = constants::Action {
                                            kind: constants::ActionTypes::JOB_SUCCESS,
                                            payload: response.payload,
                                            sender: None,
                                        };
                                        let latency_print = format!("[{}]", resp.latency).bold();
                                        println!("{} {} | {} | {}", latency_print, response.payload, resp.isp.unwrap().as_str(), resp.country.unwrap().as_str());
                                        let _ = sender.send(response);
                                    }
                                    false => {
                                        let response = constants::Action {
                                            kind: constants::ActionTypes::JOB_FAILED,
                                            payload: response.payload,
                                            sender: None,
                                        };
                                        let _ = sender.send(response);
                                    }
                                }
                            }
                            constants::ActionTypes::NO_AVAILABLE_JOB => {
                                break;
                            }
                        }
                    }
                    Err(_) => {}
                }
                thread::sleep(Duration::from_millis(50));
            }
        });

        child_threads.push(thread);
    }

    loop {
        match receiver.recv() {
            Ok(response) => {
                match response.kind {
                    // Unhandled Actions
                    constants::ActionTypes::JOB_REQUIRE_APPROVED => {}
                    constants::ActionTypes::NO_AVAILABLE_JOB => {}
                    constants::ActionTypes::JOB_SUCCESS => {}
                    constants::ActionTypes::JOB_FAILED => {}
                    // Handle Actions
                    constants::ActionTypes::REQUIRE_JOB => {
                        // let proxy_list = proxy_list;
                        let proxy_len = proxy_list.len();
                        if proxy_len > 0 {
                            let resp = constants::Action {
                                kind: constants::ActionTypes::JOB_REQUIRE_APPROVED,
                                payload: proxy_list[proxy_len - 1].clone(),
                                sender: None,
                            };
                            let _ = response.sender.unwrap().send(resp);
                            let _ = &mut proxy_list.pop();
                        } else {
                            let resp = constants::Action {
                                kind: constants::ActionTypes::NO_AVAILABLE_JOB,
                                payload: "".to_string(),
                                sender: None,
                            };
                            let _ = response.sender.unwrap().send(resp);
                            terminated_threads = terminated_threads + 1;
                        }
                    }
                }
            }
            Err(_) => println!("Error."),
        }

        if terminated_threads == child_threads.len() {
            println!("The end is here. Goodbye");
            break;
        }
    }
}
