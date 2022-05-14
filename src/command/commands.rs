use super::NewCommand;

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    /// 생성
    New(NewCommand),
}
