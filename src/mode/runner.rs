
use regex::Regex;
use std::thread::sleep;
use rand::prelude::*;
use serde::{Deserialize};
use std::time::{Duration, Instant};

use crate::constants::{self, INITIAL_RESULT, Result};

#[derive(Deserialize)]
pub struct IpResponse {
    city: String,
    country: String,
    countryCode: String,
    isp: String,
    lat: f32,
    lon: f32,
    org: String,
    query: String,
    region: String,
    regionName: String,
    status: String,
    timezone: String,
    zip: String,
}

pub fn result_adapter(latency: u128, proxy_string: String, response: IpResponse) -> Result {

    let proxy_regex = Regex::new(constants::PROXY_REGEX).unwrap();

    let regex_result = proxy_regex.captures(&proxy_string).unwrap();

    let result: Result = Result {
        latency,
        success: true,
        proxyType: Some(String::from(&regex_result[1])),
        ip: Some(String::from(&regex_result[2])),
        port: Some(String::from(&regex_result[3])),
        city: Some(response.city),
        country: Some(response.country),
        countryCode: Some(response.countryCode),
        isp: Some(response.isp),
        lat: Some(response.lat),
        lon: Some(response.lon),
        org: Some(response.org),
        query: Some(response.query),
        region: Some(response.region),
        regionName: Some(response.regionName),
        status: Some(response.status),
        timezone: Some(response.timezone),
        zip: Some(response.zip),    
    };

    result
}

pub fn check_proxy(proxy_string: String) -> Result {
    // Generate random IDLE time
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0, 1);
    let ide_duration = Duration::new(random_number, 0);
    sleep(ide_duration);
    //
    let proxy = reqwest::Proxy::all(&proxy_string).expect("Failed to build proxy");
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .timeout(Duration::from_secs(3))
        .danger_accept_invalid_certs(true)
        .proxy(proxy)
        .build()
        .expect("Cannot create Client");
    let runtime = tokio::runtime::Runtime::new();

    match runtime {
        Ok(runtime) => {
            let mut runtime = runtime;
            let response = runtime.block_on(async move {
                let current_time = Instant::now();
                let resp = client.get("http://ip-api.com/json").send().await;
                match resp {
                    Ok(response) => {
                        let response_json = response.json::<IpResponse>().await;

                        match response_json {
                            Ok(response_json) => {
                                return result_adapter(current_time.elapsed().as_millis(), proxy_string, response_json);
                            }
                            Err(_) => {
                                return INITIAL_RESULT;
                            }
                        }
                    }
                    Err(_) => {
                        return INITIAL_RESULT;
                    }
                }
            });
            return response;
        }
        Err(_) => {
            return INITIAL_RESULT;
        }
    };
}
