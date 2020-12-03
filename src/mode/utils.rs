// Import libraries, I known this useless but this is for splitting parts of code.

extern crate reqwest;
extern crate tokio;

use colored::*;
use console::Term;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use rusqlite::{ Connection, params };
use indicatif::{ProgressBar, ProgressIterator};
use crate::constants;
use std::path::Path;
use regex::Regex;

pub fn save_proxy(success_proxy_list: Vec<String>) {

  let mut write_buffer = String::new();

  for proxy in success_proxy_list {
    let proxy_str = format!("{}\n", proxy);
    let new_buffer = format!("{}{}", write_buffer, proxy_str);
    write_buffer = new_buffer;
  }

  let mut file = File::create("result.txt").unwrap();

  file.write(write_buffer.as_bytes()).expect("Failed to write result to file");

}

pub fn get_latency_color(latency: u128) -> colored::ColoredString {
  let latency_print = format!("[{}ms]", latency).bold();

  if latency <= 200 {
      return latency_print.green();
  } else if latency <= 500 {
      return latency_print.yellow();
  } else {
      return latency_print.red();
  }
}

pub fn check_proxy_string(proxy_string: String) -> Option<String> {

  let proxy_regex = Regex::new(constants::PROXY_REGEX).unwrap();
  let regex_result = proxy_regex.captures(&proxy_string).expect("Unsupported proxy format have been found");

  match &regex_result[1] {
    "http" => {
      let proxy_string = format!("{}://{}:{}", &regex_result[1], &regex_result[2], &regex_result[3]);
      Some(proxy_string)
    }
    "socks5" => {
      let proxy_string = format!("{}://{}:{}", &regex_result[1], &regex_result[2], &regex_result[3]);
      Some(proxy_string)
    }
    _ => None,
  }

}

pub fn read_proxy_file(term: &Term, path: &str, proxy_type: String, mut proxy_list: Vec<String>) -> Vec<String> {
  // Create a path to the desired file
  let path = Path::new(path);
  let display = path.display();
  // Open the path in read-only mode, returns `io::Result<File>`
  let file = match File::open(&path) {
      Err(why) => panic!("Couldn't open {}: {:?}", display, why.kind()),
      Ok(file) => file,
  };

  let buffer = BufReader::new(file);
  let lines = buffer.lines();

  let proxy_read_progress = ProgressBar::new_spinner();

  proxy_read_progress.set_message("Progressing your proxy file");

  for line in lines.progress_with(proxy_read_progress) {
      match line {
          Ok(line) => {
            let proxy_string = format!("{}://{}", proxy_type, line);
            
            match check_proxy_string(proxy_string) {
              Some(proxy_string) => {
                proxy_list.insert(proxy_list.len(), proxy_string);
              },
              None => {}
            }
          }
          Err(_) => {
          }
      }
  }

  term.clear_last_lines(2).unwrap();

  proxy_list
}

pub fn ger_proxy_from_sqlite(conn: &Connection, mut proxy_list: Vec<String>) -> Vec<String> {
  
  let mut proxy_query = conn.prepare("SELECT * from proxy").unwrap(); 

  let proxy_iter = proxy_query.query_map(params![], |row| {

    let latency: i32 = row.get(5).unwrap();

    Ok(constants::SQLiteProxy {
      id: row.get(0).unwrap(),
      proxy_type: row.get(1).unwrap(),
      ip: row.get(2).unwrap(),
      port: row.get(3).unwrap(),
      country: row.get(4).unwrap(),
      isp: row.get(6).unwrap(),
      latency,
    })
  }).unwrap();

  for proxy in proxy_iter {

    match proxy {
      Ok(sqlite_proxy) => {
        let proxy_string = format!("{}://{}:{}", sqlite_proxy.proxy_type, sqlite_proxy.ip, sqlite_proxy.port);
        proxy_list.insert(proxy_list.len(), proxy_string);    
      },
      Err(_) => {},
    }

  }

  proxy_list

}

// Initialize Things

pub fn initialize_sqlite(dbname: &str) -> Connection {
  let conn = Connection::open(dbname).unwrap();

  conn.execute("CREATE TABLE IF NOT EXISTS proxy (
      id INTEGER PRIMARY KEY,
      proxy_type TEXT NOT NULL,
      ip TEXT NOT NULL,
      port TEXT NOT NULL,
      country TEXT NOT NULL,
      latency INTEGER NOT NULL,
      isp TEXT NOT NULL
    )", params![]).expect("Error while creating database");

  conn
}

pub fn insert_proxy_to_database(conn: &Connection, proxy_info: constants::Result) {

  let proxy_type: String = proxy_info.proxyType.unwrap();
  let ip: String = proxy_info.ip.unwrap();
  let port: String = proxy_info.port.unwrap();
  let country: String = proxy_info.country.unwrap();
  let isp: String = format!("{}", proxy_info.isp.unwrap().clone());
  let latency: String = format!("{}", proxy_info.latency);

  let _ = conn.execute("INSERT INTO proxy (proxy_type, ip, port, country, isp, latency) VALUES (:proxy_type, :ip, :port, :country, :isp, :latency)",
  params![
      &proxy_type,
      &ip, 
      &port,
      &country,
      &isp,
      &latency,
      ]).unwrap();
}

pub fn show_result(term: &Term, success: i32, failed: i32) {

  let success_str = format!("{}", success);
  let failed_str = format!("{}", failed);

  term.write_line("\n").unwrap();
  term.write_line("\n").unwrap();
  let checked_string = format!("Checking progress is done, you have: {} working out of {} non-working proxies", success_str.bold().green(), failed_str.bold().red());
  term.write_line(&checked_string).unwrap();
  term.write_line("\n").unwrap();
}