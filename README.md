# Garfield Proxy Checker

A simple proxy checker that built with Rust. It come with 2 mode, first is check from your file, second is re-check previous valid proxies

## Getting Started

I will assume you already have cargo setup in your machine, if not, just follow this tutorial [rustup.sh](https://rustup.rs/)

1. Clone this repo `git clone https://github.com/GarfDev/proxy_checker.git`

2. Inside project, run command `cargo run`

3. Follow instruction in your terminal to see how program work, in case you choose option 1, I already prepared `proxy.txt` and `socks5.txt` file for testing purpose, you just need to copy and paste path navigate to those file and select correct proxy mode.

Result will come with following format:

```
[1872ms] http://124.41.213.201:39272 | WorldLink Communications | Nepal
// [latency] proxy_ip_and_port | Provider | Country
```

Thanks for reading, have fun with this simple program