use structopt::StructOpt;

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
    }
}