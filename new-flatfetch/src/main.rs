#[macro_use]

mod request_sender;

use request_sender::*;
use std::thread;
use clap::{App, Arg};

#[tokio::main]
async fn main() {
let matches = App::new("FlatFetch")
// Might use yaml instead in future versions
      .version("0.1.0")
      .author("Matteo Rosato (MRRCode979 on github) <matteorosato979@gmail.com>")
      .about("flatfetch is a wget clone (sort of) written in Rust that downloads files. I have a plan to make a package manager in Rust in the future!

License GPLv3+: GNU GPL version 3 or later
flatfetch  Copyright (C) 2022  Matteo Rosato
This program comes with ABSOLUTELY NO WARRANTY; This is free software, and you are welcome to redistribute it under certain conditions")
      .arg(Arg::new("URL")
      .help("Url = [URL for the file (must begin in http:// https:// or else it wont work) ]")
      .required(true)
      .takes_value(true))
      .get_matches();
      let _url = matches.value_of("URL").unwrap();
      
      request_sender::fetch_url(_url).await;
}
