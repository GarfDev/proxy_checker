extern crate tokio;

use std::fs::File;
use std::thread;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader, Write};
use std::sync::mpsc::{channel, Sender};
use std::path::Path;
use std::time::Duration;

enum ActionTypes {
    // Client Types
    REQUIRE_JOB,
    JOB_SUCCESS,
    JOB_FAILED,
    // Server Types
    NO_AVAIABLE_JOB,
    JOB_REQUIRE_APPROVED,
}

struct Action {
    kind: ActionTypes,
    payload: String,
    sender: Option<Sender<Action>>,
}


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
            Err(_error) => {
                // println!("ERROR: {:?}", error.kind())
            },
        }
    }

    proxy_list
}

fn check_proxy(proxy_string: String) -> bool {
    let proxy = reqwest::Proxy::all(&proxy_string).expect("Failed to build proxy");
    let client = reqwest::Client::builder().timeout(Duration::from_secs(3)).danger_accept_invalid_certs(true).proxy(proxy).build().expect("Cannot create Client");
 
    let runtime = tokio::runtime::Runtime::new();

    match runtime {
        Ok(runtime) => {
            let mut runtime = runtime;
            let response = runtime.block_on(async move {
                let resp = client.get("https://api.ipify.org?format=json").send().await;
                match resp {
                    Ok(_) => {
                        return true;
                    },
                    Err(_) => {
                        return false;
                    }
                }        
            });
            
            return response;
        },
        Err(_) => {
            return false;
        }
    };

}


fn main() {

    let mut input = String::new();
    print!("Please enter path: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    let proxy_list = vec![];
    // println!("{:?}", input.as_ptr());
    let mut proxy_list = read_and_add(&input[0..input.len() - 2].to_string() ,proxy_list); // <--- proxy_list moved to here and returned by the function
    // let proxy_list = Mutex::from(proxy_list);
    let mut child_threads = vec![];
    let (sender, receiver) = channel::<Action>();

    for i in 0..30 {
        // let list_clone_num = list_arc_num.clone();
        let sender = sender.clone();
        let thread = thread::spawn(move || {
            let (tx, rx) = channel::<Action>();

            loop {
                thread::sleep(Duration::from_secs(1));
                let tx = tx.clone();
                let response = Action {
                    kind: ActionTypes::REQUIRE_JOB,
                    payload: format!("Send from thread {}", i).to_string(),
                    sender: Some(tx),
                };    
                let _ = sender.send(response);
                match rx.recv() {
                    Ok(response) => {
                        match response.kind {
                            // Unhandle Actions
                            ActionTypes::REQUIRE_JOB => {},
                            ActionTypes::JOB_SUCCESS => {},
                            ActionTypes::JOB_FAILED => {},
                            // Handle Actions
                            ActionTypes::JOB_REQUIRE_APPROVED => {
                                let proxy_string = format!("http://{}", response.payload);
                                let resp = check_proxy(proxy_string);
                                
                                match resp {
                                    true => {
                                        let response = Action {
                                            kind: ActionTypes::JOB_SUCCESS,
                                            payload: response.payload,
                                            sender: None,
                                        };
                                        let _ = sender.send(response);
                                    },
                                    false => {
                                        let response = Action {
                                            kind: ActionTypes::JOB_FAILED,
                                            payload: response.payload,
                                            sender: None,
                                        };
                                        let _ = sender.send(response);
                                    },
                                }

                            },
                            ActionTypes::NO_AVAIABLE_JOB => {
                                println!("Thread {}: No jobs for me, I will sucide", i);
                                break;
                            },
                        }
                    }
                    Err(_) => {}
                }    
                thread::sleep(Duration::from_millis(50));
            }
        });

        child_threads.push(thread);
};

loop {
    match receiver.recv() {
        Ok(response) => {
            // println!("{:?}", response.payload);
            
            match response.kind {
                // Unhandle Actions
                ActionTypes::JOB_REQUIRE_APPROVED => {},
                ActionTypes::NO_AVAIABLE_JOB => {},
                // Handle Actions
                ActionTypes::REQUIRE_JOB => {
                    // let proxy_list = proxy_list;
                    let proxy_len = proxy_list.len();
                    if proxy_list.len() > 0 {
                        let resp = Action {
                            kind: ActionTypes::JOB_REQUIRE_APPROVED,
                            payload: proxy_list[proxy_len-1].clone(),
                            sender: None,
                        };
                        let _ = response.sender.unwrap().send(resp);
                    };
                    let _ = &mut proxy_list.pop();
                },
                ActionTypes::JOB_SUCCESS => {
                    println!("[WORKING] {}", response.payload);
                },
                ActionTypes::JOB_FAILED => {
                    println!("[NON-WORKING] {}", response.payload);
                },
            }
        }
        Err(_) => println!("Error."),
    }
    // println!("Current Thread: {}", child_threads.len())    
}
    // println!("{:?}", list_arc_num);


}
