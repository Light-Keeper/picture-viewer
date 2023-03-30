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

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Command::Describe(a), Command::Describe(b)) => a.path == b.path,
            (Command::Show(a), Command::Show(b)) => a.path == b.path,
            _ => false,
        }
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe() {
        let args = Args::parse_from(&["", "describe", "path/to/file"]);
        assert_eq!(
            args.cmd,
            Command::Describe(Describe {
                path: "path/to/file".to_string()
            })
        );
    }

    #[test]
    fn test_show() {
        let args = Args::parse_from(&["", "show", "path/to/file"]);
        assert_eq!(
            args.cmd,
            Command::Show(Show {
                path: "path/to/file".to_string()
            })
        );
    }
}
