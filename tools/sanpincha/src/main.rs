mod utils;

use std::fs;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
    multipart,
};
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
    url: String,
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
struct PutJsonDataResponse {
    message: String,
    path: String,
}

async fn get_data<T: for<'de> Deserialize<'de>>(path: String) -> T {
    let config = utils::get_config();

    let request_url = format!("{}{}", config.base_url, path);
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

async fn put_json_data<T: Serialize>(path: String, req_body: &T) -> PutJsonDataResponse {
    let config = utils::get_config();

    let request_url = format!("{}{}", config.base_url, path);
    let auth_header = format!(
        "Basic {}",
        utils::encode_base64(&format!(
            "{}:{}",
            config.basic_auth_username, config.basic_auth_password
        ))
    );
    let client = reqwest::Client::new();

    let response = client
        .put(&request_url)
        .header(USER_AGENT, "sanpincha")
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, &auth_header)
        .json(req_body)
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status().is_success() => res,
        _ => panic!("Request failed"),
    };

    let res_data = response
        .json::<PutJsonDataResponse>()
        .await
        .expect("Something went wrong while parsing");

    res_data
}

struct FileData {
    name: String,
    content: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PutFileDataResponse {
    uploaded: String,
    key: String,
}

async fn put_file_data(path: String, file_data: FileData) -> PutFileDataResponse {
    let config = utils::get_config();

    let request_url = format!("{}{}", config.base_url, path);
    let auth_header = format!(
        "Basic {}",
        utils::encode_base64(&format!(
            "{}:{}",
            config.basic_auth_username, config.basic_auth_password
        ))
    );
    let client = reqwest::Client::new();

    let part = multipart::Part::bytes(file_data.content).file_name(file_data.name.clone());
    let form = reqwest::multipart::Form::new()
        .text("name", file_data.name)
        .part("file", part);

    let response = client
        .put(&request_url)
        .header(USER_AGENT, "sanpincha")
        .header(AUTHORIZATION, &auth_header)
        .multipart(form)
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status().is_success() => res,
        _ => panic!("Request failed"),
    };

    let res_data = response
        .json::<PutFileDataResponse>()
        .await
        .expect("Something went wrong while parsing");

    res_data
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Equivalent to GET request
    Get {
        path: String,
    },
    /// Equivalent to PUT request
    Put {
        path: String,

        #[clap(short = 'f', long = "file", help = "File path")]
        file: Option<String>,
    },
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
        Commands::Put { path, file } => match path.as_str() {
            "/admin/works" => {
                let json_data = utils::get_local_json_data("assets/data/works.json");
                let data = put_json_data(path, &json_data).await;

                println!("-------------------------");
                println!("message: {:?}", data.message);
                println!("path: {:?}", data.path);
            }
            "/admin/work-history" => {
                let json_data = utils::get_local_json_data("assets/data/work_history.json");
                let data = put_json_data(path, &json_data).await;

                println!("-------------------------");
                println!("message: {:?}", data.message);
                println!("path: {:?}", data.path);
            }
            "/admin/images/favorites" => {
                let filepath = file.expect("file is required");
                let filename = filepath.split('/').last().unwrap().to_string();
                let file_data = fs::read(filepath).expect("Unable to read file");

                let data = put_file_data(
                    path,
                    FileData {
                        name: filename,
                        content: file_data,
                    },
                )
                .await;

                println!("-------------------------");
                println!("uploaded: {:?}", data.uploaded);
                println!("key: {:?}", data.key);
            }
            _ => {
                println!("Invalid path");
            }
        },
    }
}
