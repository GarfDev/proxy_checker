#![allow(dead_code)]

use std::sync::mpsc::Sender;

pub enum ActionTypes {
  // Slave States
  REQUIRE_JOB,
  JOB_SUCCESS,
  JOB_FAILED,
  // Master States
  NO_AVAILABLE_JOB,
  JOB_REQUIRE_APPROVED,
}

pub struct Action {
  pub kind: ActionTypes,
  pub payload: String,
  pub sender: Option<Sender<Action>>,
}

pub struct Combo {
  pub email: String,
  pub password: String,
}

pub struct Result {
  pub success: bool,
  pub latency: u128,
  pub ip: Option<String>,
  pub port: Option<String>,
  pub city: Option<String>,
  pub country: Option<String>,
  pub countryCode: Option<String>,
  pub isp: Option<String>,
  pub lat: Option<f32>,
  pub lon: Option<f32>,
  pub org: Option<String>,
  pub query: Option<String>,
  pub region: Option<String>,
  pub regionName: Option<String>,
  pub status: Option<String>,
  pub timezone: Option<String>,
  pub zip: Option<String>,
}

pub const INITIAL_RESULT: Result = Result {
  success: false,
  ip: None,
  latency: 0,
  port: None,
  city: None,
  country: None,
  countryCode: None,
  isp: None,
  lat: None,
  lon: None,
  org: None,
  query: None,
  region: None,
  regionName: None,
  status: None,
  timezone: None,
  zip: None,
};


// Flexing Flexing

pub const LOGO: &str = r#"
                     <-. (`-')_ <-. (`-')   (`-')  _           <-. (`-')   (`-')  _                       (`-')  _ <-. (`-')  
_             .->      \( OO) )   \(OO )_  ( OO).-/     .->      \(OO )_  (OO ).-/      .->    _         (OO ).-/    \(OO )_ 
\-,-----.(`-')----. ,--./ ,--/ ,--./  ,-.)(,------.(`-')----. ,--./  ,-.) / ,---.  ,--.(,--.   \-,-----. / ,---.  ,--./  ,-.)
 |  .--./( OO).-.  '|   \ |  | |   `.'   | |  .---'( OO).-.  '|   `.'   | | \ /`.\ |  | |(`-')  |  .--./ | \ /`.\ |   `.'   |
/_) (`-')( _) | |  ||  . '|  |)|  |'.'|  |(|  '--. ( _) | |  ||  |'.'|  | '-'|_.' ||  | |(OO ) /_) (`-') '-'|_.' ||  |'.'|  |
||  |OO ) \|  |)|  ||  |\    | |  |   |  | |  .--'  \|  |)|  ||  |   |  |(|  .-.  ||  | | |  \ ||  |OO )(|  .-.  ||  |   |  |
(_'  '--'\  '  '-'  '|  | \   | |  |   |  | |  `---.  '  '-'  '|  |   |  | |  | |  |\  '-'(_ .'(_'  '--'\ |  | |  ||  |   |  |
  `-----'   `-----' `--'  `--' `--'   `--' `------'   `-----' `--'   `--' `--' `--' `-----'      `-----' `--' `--'`--'   `--'
"#;

pub const TEXTLINE: &str = "  FLEXING FLEXING HO KHAU HOAN KIEM \n\n\n";
