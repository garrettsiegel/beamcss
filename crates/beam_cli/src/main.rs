#![forbid(unsafe_code)]

use std::{env, process};

mod args;
mod commands;
mod config;
mod init;
mod output;
mod scanner;

#[cfg(test)]
mod tests;

fn main() {
    if let Err(error) = args::run(env::args().skip(1).collect()) {
        eprintln!("beam: {error}");
        process::exit(1);
    }
}
