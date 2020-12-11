use std::io::{Cursor, Read};
use std::collections::{HashMap};

use std::borrow::{ToOwned};

use zip::{ZipArchive, result::ZipError};

mod cargo_struct;
use cargo_struct::{CargoToml};

//const TEMP_FILE_NAME: &str = "./.__actix_generator_temp.zip";

async fn get_zip() -> Result<ZipArchive<Cursor<Vec<u8>>>, ZipError> {
    let url = "https://codeload.github.com/myyrakle/actix-templates/zip/main";
    let bytes: Vec<u8> = reqwest::get(url).await.unwrap().bytes().await.unwrap().into_iter().collect();
    let cursor = Cursor::new(bytes);
    
    zip::ZipArchive::new(cursor)
}

// async fn create_basic_template(project_name: String) -> Result<(), Box<dyn std::error::Error>> {

// }



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut args = std::env::args();
    args.next();
    let args:Vec<String> = args.collect();

    let _options: Vec<String> = args.clone().into_iter().filter(|e| e.chars().nth(0).unwrap() == '-').collect();
    let values: Vec<String> = args.into_iter().filter(|e| e.chars().nth(0).unwrap() != '-').collect();

    let project_name = values[0].clone();

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

    //std::fs::create_dir(&project_name).expect("실패");
    let basic = templates.get("basic").unwrap().clone();

    for (path, data) in basic.into_iter() {
        //println!("수정 전 경로: {}", path);

        let path = [project_name.clone(), path].join("/");

        //println!("경로: {}", path);
        if data.is_some() {
            std::fs::write(path, data.unwrap()).expect("실패");
        } 
        else {
            std::fs::create_dir(path).expect("실패");
        }
    }

    let cargo_toml_path = [project_name.clone(), "Cargo.toml".to_owned()].join("/");
    let cargo_toml_text = std::fs::read_to_string(&cargo_toml_path).unwrap();
    let mut cargo_toml:CargoToml = toml::from_str(&cargo_toml_text).unwrap();
    cargo_toml.set_name(project_name.clone());
    println!("{:?}", cargo_toml);
    std::fs::write(&cargo_toml_path, toml::to_string(&cargo_toml).unwrap()).unwrap();

    Ok(())
}
