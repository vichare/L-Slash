use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
pub struct Flags {
    #[arg(short = 'p', long = "dbpath")]
    pub db_path: std::path::PathBuf,
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Action {
    List,
    MoveRecords,
    Lookup(LookupAction),
    Add(AddAction),
}

#[derive(Args, Debug, PartialEq)]
pub struct LookupAction {
    pub alias: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct AddAction {
    pub alias: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_list() {
        let flags = Flags::parse_from(vec!["cli", "-p", ".", "list"]);
        assert_eq!(flags.action, Action::List);
        assert_eq!(flags.db_path, std::path::PathBuf::from("."));
    }
}
