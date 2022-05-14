use crate::lib::toml::CargoToml;
use crate::lib::{TemplateError, TEMPLATE_URL};
use std::io::{Cursor, Read};

use std::borrow::ToOwned;

use zip::ZipArchive;

pub struct TemplateManager {
    project_name: String,
    template_name: String,
}

impl TemplateManager {
    pub fn new(project_name: String, template_name: String) -> Self {
        Self {
            project_name,
            template_name,
        }
    }

    // 템플릿 소스코드를 zip으로 획득
    async fn get_zip() -> Result<ZipArchive<Cursor<Vec<u8>>>, Box<dyn std::error::Error>> {
        let bytes: Vec<u8> = reqwest::get(TEMPLATE_URL)
            .await?
            .bytes()
            .await?
            .into_iter()
            .collect();
        let cursor = Cursor::new(bytes);

        match zip::ZipArchive::new(cursor) {
            Err(error) => Err(TemplateError::boxed(error.to_string())),
            Ok(zip) => Ok(zip),
        }
    }

    async fn get_template(
        &self,
    ) -> Result<Vec<(String, Option<Vec<u8>>)>, Box<dyn std::error::Error>> {
        let mut zip = Self::get_zip().await?;

        let target_template_name = self.template_name.clone();
        let mut file_list = vec![];

        for i in 0..zip.len() {
            let file = zip.by_index(i)?;

            let template_name = file.name().split("/").nth(2).map(ToOwned::to_owned);
            let is_template = file.name().split("/").nth(3).is_some();

            let mut split = file.name().split("/");
            split.next();
            split.next();
            split.next();
            let path: Vec<&str> = split.collect();
            let path = path.join("/");

            let data = if file.is_file() {
                let data: Vec<u8> = file
                    .bytes()
                    .filter(|e| e.is_ok())
                    .map(|e| e.unwrap())
                    .collect();

                let data = if path == "Cargo.toml" {
                    let cargo_toml = String::from_utf8(data)?;
                    let cargo_toml = self.edit_cargo_toml(cargo_toml)?;
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
                    if target_template_name == template_name {
                        file_list.push(file_value);
                    }
                }
            }
        }

        Ok(file_list)
    }

    fn edit_cargo_toml(&self, source: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut cargo_toml: CargoToml = toml::from_str(&source)?;
        cargo_toml.set_name(self.project_name.clone());
        Ok(toml::to_string(&cargo_toml)?)
    }

    pub async fn new_template(&self, base_path: String) -> Result<(), Box<dyn std::error::Error>> {
        let template = self.get_template().await?;

        for (path, data) in template.into_iter() {
            let path = [base_path.clone(), path].join("/");

            if data.is_some() {
                std::fs::write(&path, data.unwrap())?;
                println!(">>>>> {} >>> file created", path);
            } else {
                match std::fs::create_dir(&path) {
                    Err(error) => {
                        if error.kind() == std::io::ErrorKind::AlreadyExists {
                            println!(">>>>> {} >>> directory already exists", path);
                        } else {
                            return Err(TemplateError::boxed(error.to_string()));
                        }
                    }
                    Ok(_) => println!(">>>>> {} >>> directory created", path),
                }
            }
        }

        println!("#### Generation Success ####");

        Ok(())
    }
}
