
use serde::{Deserialize};
use std::time::{Duration, Instant};

use crate::constants::{INITIAL_RESULT, Result};

#[allow(dead_code)]
#[derive(Deserialize)]
struct IP_RESPONSE {
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

fn result_adapter(latency: u128, proxy_string: String, response: IP_RESPONSE) -> Result {
    let proxy_string: Vec<&str> = proxy_string.split(":").collect();

    let result: Result = Result {
        latency,
        success: true,
        ip: Some(proxy_string[0].to_string()),
        port: Some(proxy_string[1].to_string()),
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
                        let response_json = response.json::<IP_RESPONSE>().await;

                        match response_json {
                            Ok(response_json) => {
                                // println!("{:#?}", response_json.country);
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
