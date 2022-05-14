use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct NewCommandOption {
    /// Template name
    #[clap(long, short)]
    pub template_name: Option<String>,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "new")]
pub struct NewCommand {
    /// Project name
    #[clap(name = "PROJECT_NAME")]
    pub project_name: String,

    #[clap(flatten)]
    pub option: NewCommandOption,
}
