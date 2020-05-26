use structopt::StructOpt;

use crate::com::errors::EphError;

#[derive(StructOpt)]
pub(crate) struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command
}


#[derive(StructOpt)]
#[structopt(about = "Cast a file into the xe filesystem")]
pub(crate) enum Command {
    Init {
        #[structopt(long)]
        standalone: bool
    },
    Cast {
        path: String,
    },
}

/// Check if specified can was initiated on local machine
pub(crate) fn is_init(can: &str) -> Result<(), EphError> {
    if can == "ok" {
        Ok(())
    } else {
        Err(EphError::new("Can is not initiated."))
    }
}