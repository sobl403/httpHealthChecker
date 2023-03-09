use reqwest::StatusCode;
use std::{env, io::{self, Write}, thread, time};

fn main() {
    // Get the interval and URL arguments from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: healthcheck <interval> <url>");
        std::process::exit(1);
    }
    let interval = args[1].parse::<u64>().unwrap();
    let url = args[2].parse::<reqwest::Url>().unwrap();

    loop {
        let res = reqwest::blocking::get(url.clone());
        match res {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => println!("OK(200)"),
                    status => println!("ERR({})", status.as_u16()),
                }
            },
            Err(_) => println!("ERR(Connection error)"),
        }

        // Manually flush the output buffer to ensure that the output is printed immediately
        io::stdout().flush().unwrap();

        thread::sleep(time::Duration::from_millis(interval));
    }
}