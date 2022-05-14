use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct NewCommandOption {
    /// 생성할 템플릿 이름
    #[clap(long, short)]
    pub template_name: Option<String>,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "new")]
pub struct NewCommand {
    /// 생성될 프로젝트 이름
    #[clap(name = "PROJECT_NAME")]
    pub project_name: String,

    #[clap(flatten)]
    pub option: NewCommandOption,
}
