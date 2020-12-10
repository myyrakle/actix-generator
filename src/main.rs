use serde::{Deserialize};
use reqwest::{Client};

#[derive(Debug, Deserialize)]
struct GithubResponse
{
    paths: Vec<String>, 
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let url = "https://github.com/myyrakle/actix-templates/tree-list/44192773f8427e170b74249932b989f2b623ed8e";

    let response = reqwest::get(url).await?.status();//json().await?;

    let foo = reqwest::Client::new().get(url).header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.198 Safari/537.36").send().await?.text().await?;

    println!("{:?}", foo);

    Ok(())
}
