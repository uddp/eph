use std::fs::File;
use std::io;

use sha2::{Digest, Sha256};
use structopt::StructOpt;

mod fs;
mod config;
mod cli;

fn main() {
    let _opt = cli::Cli::from_args();
    match _opt.cmd {
        cli::Command::Init { standalone } => {
            fs::init(standalone)
        }
        cli::Command::Cast { path } => {
            let mut file = File::open(path).expect("Unable to open");
            let mut sha256 = Sha256::new();
            io::copy(&mut file, &mut sha256).expect("Unable to open");
            let hash = sha256.result();
            println!("hash is: {:x}", hash);
        }
    }
}
