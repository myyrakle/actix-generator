use super::{InitCommand, NewCommand};

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    /// Create new prodject
    New(NewCommand),
    /// Init new project
    Init(InitCommand),
}
