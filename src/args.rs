use clap::Parser;

#[derive(Parser, Debug)]
pub struct Describe {
    pub path: String,
}

#[derive(Parser, Debug)]
pub struct Show {
    pub path: String,
}

#[derive(Parser, Debug)]
pub enum Command {
    #[clap(name = "describe")]
    Describe(Describe),
    #[clap(name = "show")]
    Show(Show),
}

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
}
