#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CargoToml {
    package: CargoPackage, 
    dependencies: toml::value::Value,
    features: Option<toml::value::Value>, 
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CargoPackage {
    name: String, 
    version: toml::value::Value, 
    authors: toml::value::Value,
    edition: toml::value::Value,
}

impl CargoPackage 
{
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl CargoToml 
{
    pub fn set_name(&mut self, name: String) {
        self.package.set_name(name);
    }
}