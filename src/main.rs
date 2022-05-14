mod command;
mod lib;

use clap::Parser;
use command::args::Args;
use lib::TemplateManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    match args.action {
        command::SubCommand::New(command) => {
            let project_name = command.project_name;
            let template_name = command.option.template_name.unwrap_or("basic".to_owned());
            let template_manager = TemplateManager::new(project_name, template_name);

            template_manager.new_template().await?;
        }
        _ => {}
    }

    Ok(())
}
