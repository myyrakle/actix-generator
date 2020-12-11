use std::io::{Cursor, Read};
use std::collections::{HashMap};

use std::borrow::{ToOwned};

use zip::{ZipArchive, result::ZipError};

//const TEMP_FILE_NAME: &str = "./.__actix_generator_temp.zip";

async fn get_zip() -> Result<ZipArchive<Cursor<Vec<u8>>>, ZipError> {
    let url = "https://codeload.github.com/myyrakle/actix-templates/zip/main";
    let bytes: Vec<u8> = reqwest::get(url).await.unwrap().bytes().await.unwrap().into_iter().collect();
    let cursor = Cursor::new(bytes);
    
    zip::ZipArchive::new(cursor)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let args:Vec<String> = std::env::args().collect();

    let _options: Vec<String> = args.clone().into_iter().filter(|e| e.chars().nth(0).unwrap() == '-').collect();
    let values: Vec<String> = args.into_iter().filter(|e| e.chars().nth(0).unwrap() != '-').collect();

    let project_name = values[0].clone();
    //std::fs::create_dir(project_name).expect("실패");

    let mut templates = HashMap::new();

    let mut zip = get_zip().await?;

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;

        let template_name = file.name().split("/").nth(1).map(ToOwned::to_owned);

        let is_dir = file.is_dir();

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

        if is_dir {
            if let Some(template_name) = template_name {
                println!("{}", template_name);
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

    //println!("{:?}", templates);
    //std::fs::write("./foo/bar/boom.txt", b"asdf").expect("실패");

    Ok(())
}
