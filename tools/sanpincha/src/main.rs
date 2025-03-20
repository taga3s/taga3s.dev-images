mod utils;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Work {
    id: String,
    title: String,
    description: String,
    tech_stack: String,
    github_url: String,
    order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Works {
    works: Vec<Work>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FavPhoto {
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FavPhotos {
    images: Vec<FavPhoto>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkHistoryItem {
    id: String,
    span: String,
    company: String,
    description: String,
    tech_stack: String,
    order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkHistory {
    work_history: Vec<WorkHistoryItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PutResponse {
    message: String,
    path: String,
}

async fn get_data<T: for<'de> Deserialize<'de>>(path: String) -> T {
    let base_url = std::env::var("BASE_URL").expect("BASE_URL is not set");
    let request_url = format!("{}{}", base_url, path);

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "sanpincha")
        .header(ACCEPT, "application/json")
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status().is_success() => res,
        _ => panic!("Request failed"),
    };

    let res_data = response
        .json::<T>()
        .await
        .expect("Something went wrong while parsing");

    res_data
}

async fn put_data<T: Serialize>(path: String, req_body: &T) -> PutResponse {
    let username = std::env::var("BASIC_AUTH_USERNAME").expect("BASIC_AUTH_USERNAME is not set");
    let password = std::env::var("BASIC_AUTH_PASSWORD").expect("BASIC_AUTH_PASSWORD is not set");

    let base_url = std::env::var("BASE_URL").expect("BASE_URL is not set");
    let request_url = format!("{}{}", base_url, path);

    let client = reqwest::Client::new();
    let response = client
        .put(&request_url)
        .header(USER_AGENT, "sanpincha")
        .header(ACCEPT, "application/json")
        .header(
            AUTHORIZATION,
            format!(
                "Basic {}",
                utils::encode_base64(&format!("{}:{}", username, password))
            ),
        )
        .json(req_body)
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status().is_success() => res,
        _ => panic!("Request failed"),
    };

    let res_data = response
        .json::<PutResponse>()
        .await
        .expect("Something went wrong while parsing");

    res_data
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Get { path: String },
    Put { path: String },
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Get { path } => match path.as_str() {
            "/works" => {
                let data = get_data::<Works>(path).await;

                for work in data.works {
                    println!("-------------------------");
                    println!("id: {:?}", work.id);
                    println!("title: {:?}", work.title);
                    println!("description: {:?}", work.description);
                    println!("tech_stack: {:?}", work.tech_stack);
                    println!("github_url: {:?}", work.github_url);
                    println!("order: {:?}", work.order);
                }
            }
            "/work-history" => {
                let data = get_data::<WorkHistory>(path).await;
                for wh in data.work_history {
                    println!("-------------------------");
                    println!("id: {:?}", wh.id);
                    println!("span: {:?}", wh.span);
                    println!("company: {:?}", wh.company);
                    println!("description: {:?}", wh.description);
                    println!("tech_stack: {:?}", wh.tech_stack);
                    println!("order: {:?}", wh.order);
                }
            }
            "/photos/favs" => {
                let data = get_data::<FavPhotos>(path).await;
                for photo in data.images {
                    println!("-------------------------");
                    println!("uri: {:?}", photo.uri);
                }
            }
            _ => {
                println!("Invalid path");
            }
        },
        Commands::Put { path } => match path.as_str() {
            "/admin/works" => {
                let json_data = utils::get_local_json_data("assets/data/works.json");
                let data = put_data(path, &json_data).await;

                println!("-------------------------");
                println!("message: {:?}", data.message);
                println!("path: {:?}", data.path);
            }
            "/admin/work-history" => {
                let json_data = utils::get_local_json_data("assets/data/work_history.json");
                let data = put_data(path, &json_data).await;

                println!("-------------------------");
                println!("message: {:?}", data.message);
                println!("path: {:?}", data.path);
            }
            _ => {
                println!("Invalid path");
            }
        },
    }
}
