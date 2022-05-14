use crate::lib::toml::CargoToml;
use crate::lib::{TemplateError, TEMPLATE_URL};
use std::collections::HashMap;
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
        let mut templates = HashMap::new();

        let mut zip = Self::get_zip().await?;

        let target_template_name = self.template_name.clone();

        for i in 0..zip.len() {
            let file = zip.by_index(i)?;

            let template_name = file.name().split("/").nth(1).map(ToOwned::to_owned);
            println!("template_name, {:?}", template_name);
            let is_template = file.name().split("/").nth(2).is_some();

            let mut split = file.name().split("/");
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
                    if templates.contains_key(&template_name) == false {
                        templates.insert(template_name, vec![file_value]);
                    } else {
                        let template = templates.get_mut(&template_name).unwrap();
                        template.push(file_value);
                    }
                }
            }
        }

        Err(TemplateError::boxed("template not found".to_owned()))
    }

    fn edit_cargo_toml(&self, source: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut cargo_toml: CargoToml = toml::from_str(&source)?;
        cargo_toml.set_name(self.project_name.clone());
        Ok(toml::to_string(&cargo_toml)?)
    }

    pub async fn new_template(&self) -> Result<(), Box<dyn std::error::Error>> {
        let template = self.get_template().await?;

        for (path, data) in template.into_iter() {
            let path = [self.project_name.clone(), path].join("/");

            if data.is_some() {
                std::fs::write(&path, data.unwrap())?;
                println!(">>>>> {} >>> file created", path);
            } else {
                std::fs::create_dir(&path)?;
                println!(">>>>> {} >>> directory created", path);
            }
        }

        println!("#### Generation Success ####");

        Ok(())
    }
}
