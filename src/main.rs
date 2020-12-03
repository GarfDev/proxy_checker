// Allow things that I think is good for my code style

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Import libraries, I known this useless but this is for splitting parts of code.

extern crate reqwest;
extern crate tokio;

use dialoguer::{Select, theme::ColorfulTheme};
use std::{thread, time::{self, Duration}};
use console::Term;

mod constants;
mod mode;

// Mainstream

fn main() {

    let mode_label: [colored::ColoredString; 2] = constants::get_mode_label();

    // Select program mode

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .items(&mode_label)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap().unwrap();

    let modes = Vec::from(constants::MODE);

    let mut selected_mode = modes[user_selection].clone();

    // Utils

    fn changeMode(current_mode: &mut constants::Mode, mode: constants::Mode) {
        let new_mode = mode.clone();
        *current_mode = new_mode;
    }

    // Initialize

    let conn = mode::utils::initialize_sqlite("database.db");
    let term = Term::stdout();
    // Main loop

    loop {

        match selected_mode {
            constants::Mode::CHECK_CURRENT_FILE => {
                mode::check_proxy_file::check_proxy_file(&conn, &term);
                changeMode(&mut selected_mode, constants::Mode::RE_CHECK_FROM_SQLITE);
                term.write_line("Re-check (proxies from sqlite) progress will be start in next 10 minute, and will").unwrap();
                term.write_line("continually every 24 hours. result will be save to result.txt file").unwrap();
                thread::sleep(Duration::from_secs(10));
            }
            constants::Mode::RE_CHECK_FROM_SQLITE => {
                loop {
                    mode::check_sqlite_file::check_sqlite_file(&conn, &term);
                    let one_day = time::Duration::from_secs(86400);
                    term.write_line("Re-check (proxies from sqlite) progress will be start in next 24 hour, result will").unwrap();
                    term.write_line("be save to result.txt file").unwrap();
                    thread::sleep(one_day);
                    term.flush().unwrap();
                }
            }
        }

    }

}