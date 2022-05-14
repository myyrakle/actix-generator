use super::{InitCommand, NewCommand};

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    /// 생성
    New(NewCommand),
    // 초기화
    Init(InitCommand),
}
