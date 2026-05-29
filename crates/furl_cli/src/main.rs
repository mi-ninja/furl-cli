//! # `furl-cli`
//!
//! A fast, multithreaded CLI downloader built in Rust.
//!
//! ## Usage
//!
//! ```bash
//! furl [URL]
//! ```
//!

use clap::Parser;
use furl_core::{Downloader, FurlCliArgs, GraphicalProgressReporter};
use regex::Regex;
use std::process::exit;

use std::path::Path;

#[tokio::main]
async fn main() {
    let args = FurlCliArgs::parse();
    let path = Path::new(&args.out);
    let threads = args.threads;
    let filename = args.filename;

    if !path.exists() {
        println!("The destination path does not exist");
        exit(1);
    }

    // TODO: add extensive url pattern matcher
    let re = Regex::new(r"https?://[^\s/$.?#].[^\s]*").unwrap();
    if re.captures(&args.url).is_some() {
        let mut downloader =
            Downloader::new(&args.url).with_reporter(GraphicalProgressReporter::new());
        if downloader
            .download(&args.out, filename, Some(threads))
            .await
            .is_ok()
        {
            println!("Download Complete!")
        }
        return;
    }
    println!("Invalid URL provided");
    return;
}
