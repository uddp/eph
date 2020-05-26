use std::fs::File;

use sha2::digest::generic_array::GenericArray;
use sha2::digest::generic_array::typenum::U32;
use structopt::StructOpt;

mod fs;
mod config;
mod cli;
mod util;
mod com;

fn main() {
    let _opt = cli::Cli::from_args();
    match _opt.cmd {
        cli::Command::Init { standalone } => {
            fs::init(standalone)
        }
        cli::Command::Cast { path } => {
            let file = File::open(path).expect("Unable to open file.");
            let output: GenericArray<u8, U32> = util::hash::fsha256(file);
            println!("Hash: {:x}", output);
        }
    }
}
