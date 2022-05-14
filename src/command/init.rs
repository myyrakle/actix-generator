use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct InitCommandOption {
    /// Template name
    #[clap(long, short)]
    pub template_name: Option<String>,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "init")]
pub struct InitCommand {
    /// Project name
    #[clap(name = "PROJECT_NAME")]
    pub project_name: Option<String>,

    #[clap(flatten)]
    pub option: InitCommandOption,
}
