extern crate reqwest;
extern crate tokio;

use dialoguer::{Input};
use rusqlite::Connection;
use std::sync::mpsc::channel;
use std::time::Duration;
use console::Term;
use std::thread;

use colored::*;
use crate::mode::runner::*;
use crate::mode::utils::*;
use crate::constants::*;

pub fn check_proxy_file(conn: &Connection, term: &Term) {
  let proxy_list = vec![];
  let mut success_proxy_list: Vec<String> = vec![];
  let input : String = Input::new()
      .with_prompt("Drag and Drop your proxy file to here")
      .interact_text().unwrap();


  let mut proxy_list =
  read_proxy_file(&term, &input[0..input.len()].to_string(), String::from("http"), proxy_list); // <--- proxy_list moved to here and returned by the function
  
  let mut progress_proxies = 0;

  // Initialize Important things

  let mut success_count = 0;
  let mut failed_count = 0;

  // let term = Term::stdout();
  let mut child_threads = vec![];
  let mut terminated_threads = 0;
  let (sender, receiver) = channel::<Action>();

  // Progress bar

  for i in 0..300 {
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
                  result: None,
              };
              let _ = sender.send(response);
              match rx.recv() {
                  Ok(response) => {
                      match response.kind {
                          // Unhandled Actions
                          ActionTypes::REQUIRE_JOB => {}
                          ActionTypes::JOB_SUCCESS => {}
                          ActionTypes::JOB_FAILED => {}
                          // Handle Actions
                          ActionTypes::JOB_REQUIRE_APPROVED => {
                              let proxy_string = format!("{}", response.payload);
                              let resp = check_proxy(proxy_string);
                              match resp.success {
                                  true => {
                                      let response = Action {
                                          kind: ActionTypes::JOB_SUCCESS,
                                          payload: response.payload,
                                          sender: None,
                                          result: Some(resp),
                                      };
                                      let _ = sender.send(response);
                                  }
                                  false => {
                                      let response = Action {
                                          kind: ActionTypes::JOB_FAILED,
                                          payload: response.payload,
                                          sender: None,
                                          result: None,
                                      };
                                      let _ = sender.send(response);
                                  }
                              }
                          }
                          ActionTypes::NO_AVAILABLE_JOB => {
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
                  ActionTypes::JOB_REQUIRE_APPROVED => {}
                  ActionTypes::NO_AVAILABLE_JOB => {}
                  // Handle Actions
                  ActionTypes::JOB_FAILED => {
                      failed_count = failed_count + 1;
                      progress_proxies = progress_proxies + 1;
                  }
                  ActionTypes::JOB_SUCCESS => {
                      let result = response.result.unwrap();
                      let copied_result = result.clone();
                      let latency_print = get_latency_color(result.latency);
                      // TO-DO: Replace this with a function

                      let success_text = format!("{} {} | {} | {}", latency_print, response.payload.bold(), result.isp.unwrap().as_str(), result.country.unwrap().as_str());
                      insert_proxy_to_database(&conn, copied_result);
                      success_proxy_list.insert(success_proxy_list.len(), response.payload);
                      term.write_line(&success_text.as_str()).unwrap();
                      progress_proxies = progress_proxies + 1;
                      success_count = success_count + 1;
                  }
                  ActionTypes::REQUIRE_JOB => {
                      // let proxy_list = proxy_list;
                      let proxy_len = proxy_list.len();
                      if proxy_len > 0 {
                          let resp = Action {
                              kind: ActionTypes::JOB_REQUIRE_APPROVED,
                              payload: proxy_list[proxy_len - 1].clone(),
                              sender: None,
                              result: None,
                          };
                          let _ = response.sender.unwrap().send(resp);
                          let _ = &mut proxy_list.pop();
                      } else {
                          let resp = Action {
                              kind: ActionTypes::NO_AVAILABLE_JOB,
                              payload: "".to_string(),
                              sender: None,
                              result: None,
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
          show_result(&term, success_count, failed_count);
          save_proxy(success_proxy_list);
          break;
      }
  }
}
