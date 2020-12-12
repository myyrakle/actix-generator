use std::io::{Cursor, Read};
use std::collections::{HashMap};

use std::borrow::{ToOwned};

use zip::{ZipArchive, result::ZipError};

mod lib;
use lib::cargo_toml::{CargoToml};
use lib::constants::{TEMPLATE_URL, HELP_TEXT};
use lib::command::{Command};
use lib::options::{HELP_OPTIONS};

// read zip data from template repository url
async fn get_zip() -> Result<ZipArchive<Cursor<Vec<u8>>>, ZipError> {
    let bytes: Vec<u8> = reqwest::get(TEMPLATE_URL).await.unwrap().bytes().await.unwrap().into_iter().collect();
    let cursor = Cursor::new(bytes);
    
    zip::ZipArchive::new(cursor)
}

fn edit_cargo_toml(source: String, project_name: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut cargo_toml:CargoToml = toml::from_str(&source)?;
    cargo_toml.set_name(project_name);
    Ok(toml::to_string(&cargo_toml)?)
}

async fn get_templates(project_name: String) ->  Result<HashMap<String, Vec<(String, Option<Vec<u8>>)>>, Box<dyn std::error::Error>> {
    let mut templates = HashMap::new();

    let mut zip = get_zip().await?;

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;

        let template_name = file.name().split("/").nth(1).map(ToOwned::to_owned);
        let is_template = file.name().split("/").nth(2).is_some();

        let mut split = file.name().split("/");
        split.next();
        split.next();
        let path:Vec<&str> = split.collect();
        let path = path.join("/");

        let data = if file.is_file() {
            let data: Vec<u8> = file.bytes().filter(|e|e.is_ok()).map(|e|e.unwrap()).collect();

            let data = if path == "Cargo.toml" {
                let cargo_toml = String::from_utf8(data)?;
                let cargo_toml = edit_cargo_toml(cargo_toml, project_name.clone())?;
                let data: Vec<u8> = cargo_toml.bytes().collect();
                data
            } else {
                data
            };
            Some(data)
        } else {
            None
        };
        
        let file_value = (path, data);

        if is_template {
            if let Some(template_name) = template_name {
                if templates.contains_key(&template_name) == false {
                    templates.insert(template_name, vec![file_value]);
                }
                else {
                    let template = templates.get_mut(&template_name).unwrap();
                    template.push(file_value);
                }
            }
        }
    }

    Ok(templates)
}

async fn write_template(templates: HashMap<String, Vec<(String, Option<Vec<u8>>)>>, template_name: String, project_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let basic = templates.get(&template_name).unwrap().clone();

    for (path, data) in basic.into_iter() {
        //println!("수정 전 경로: {}", path);

        let path = [project_name.clone(), path].join("/");

        //println!("경로: {}", path);
        if data.is_some() {
            std::fs::write(&path, data.unwrap())?;
            println!(">>>>> {} >>> file created", path);
        } 
        else {
            std::fs::create_dir(&path)?;
            println!(">>>>> {} >>> directory created", path);
        }
    }

    println!("#### Generation Success ####");

    Ok(())
}

fn read_command() -> (Vec<String>, Vec<String>) {
    let mut args = std::env::args();
    args.next();
    args.next();
    let args:Vec<String> = args.collect();

    let options: Vec<String> = args.clone().into_iter().filter(|e| e.chars().nth(0).unwrap() == '-').collect();
    let values: Vec<String> = args.into_iter().filter(|e| e.chars().nth(0).unwrap() != '-').collect();

    (options, values)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let (options, values) = read_command();

    let command: Command = match () {
        _ if values.is_empty() && options.is_empty() => Command::Help,
        _ if options.iter().any(|e| HELP_OPTIONS.contains(&e.as_str())) => Command::Help,
        _ if options.is_empty() && values.len() >= 1 => Command::Basic,
        _ => Command::Nothing,
    };    

    match command {
        Command::Help => println!("{}", HELP_TEXT),
        Command::Basic => {
            let project_name = values[0].clone();
            let templates = get_templates(project_name.clone()).await?;
            write_template(templates, "basic".into(), project_name.clone()).await?;
        }, 
        _ => (),
    }

    Ok(())
}
