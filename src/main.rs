extern crate tokio;

use std::fs::File;
use std::thread;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader, Write};
use std::sync::{ Mutex};
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

fn check_proxy(proxy_string: String) -> bool {
    let proxy = reqwest::Proxy::all(&proxy_string).unwrap();
    let client = reqwest::Client::builder().timeout(Duration::from_secs(3)).danger_accept_invalid_certs(true).proxy(proxy).build().unwrap();

    let mut runtime = tokio::runtime::Runtime::new().unwrap();
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

    return response

}


fn main() {

    let mut input = String::new();
    print!("Please enter path: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    let proxy_list = vec![];
    // println!("{:?}", input.as_ptr());
    let proxy_list = read_and_add(&input[0..input.len() - 1].to_string() ,proxy_list); // <--- proxy_list moved to here and returned by the function
    let proxy_list = Mutex::from(proxy_list);
    let mut child_threads = vec![];
    let (sender, receiver) = channel::<Action>();

    // Mutilple Threading path

    for i in 0..100 {
        // let list_clone_num = list_arc_num.clone();
        let sender = sender.clone();
        let thread = thread::spawn(move || {
            let (tx, rx) = channel::<Action>();

            loop {
                let tx = tx.clone();
                let response = Action {
                    kind: ActionTypes::REQUIRE_JOB,
                    payload: format!("Send from thread {}", i).to_string(),
                    sender: Some(tx),
                };    
                let _ = sender.send(response).unwrap();
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
                                thread::sleep(Duration::from_secs(2));
                                let resp = check_proxy(proxy_string);
                                println!("{:?}", resp);
                            },
                            ActionTypes::NO_AVAIABLE_JOB => {
                                // println!("Thread {}: No jobs for me, I will sucide", i);
                                break;
                            },
                        }
                    }
                    Err(_) => {}
                }    
                thread::sleep(Duration::from_secs(1));
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
                    let mut proxy_list = proxy_list.lock().unwrap();
                    let proxy_len = proxy_list.len();
                    if proxy_list.len() > 0 {
                        let resp = Action {
                            kind: ActionTypes::JOB_REQUIRE_APPROVED,
                            payload: proxy_list[proxy_len-1].clone(),
                            sender: None,
                        };
                        let _ = response.sender.unwrap().send(resp);
                    };
                    let _ = proxy_list.pop();
                },
                ActionTypes::JOB_SUCCESS => {},
                ActionTypes::JOB_FAILED => {},
            }
        }
        Err(_) => println!("Error."),
    }
    // println!("Current Thread: {}", child_threads.len())    
}
    // println!("{:?}", list_arc_num);


}
