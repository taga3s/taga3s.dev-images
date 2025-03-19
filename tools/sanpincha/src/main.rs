use clap::Parser;
use dotenv::dotenv;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Cli {
    command: String,

    #[arg(short, long, default_value_t = String::from(""))]
    path: String,
}

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

async fn get_data<T: for<'de> Deserialize<'de>>(path: String) -> T {
    let base_url = std::env::var("BASE_URL").expect("BASE_URL is not set");
    let request_url = format!("{}{}", base_url, path);

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "contents-management-cli")
        .header(ACCEPT, "application/json")
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status().is_success() => res,
        _ => panic!("Request failed"),
    };

    let data = response
        .json::<T>()
        .await
        .expect("Something went wrong while parsing");

    data
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

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command.as_str() {
        "get" => match cli.path.as_str() {
            "/works" => {
                let data = get_data::<Works>(cli.path).await;

                for work in data.works {
                    println!("-------------------------");
                    println!("title: {:?}", work.title);
                    println!("description: {:?}", work.description);
                    println!("tech_stack: {:?}", work.tech_stack);
                    println!("github_url: {:?}", work.github_url);
                    println!("order: {:?}", work.order);
                }
            }
            "/work-history" => {
                let data = get_data::<WorkHistory>(cli.path).await;
                for wh in data.work_history {
                    println!("-------------------------");
                    println!("span: {:?}", wh.span);
                    println!("company: {:?}", wh.company);
                    println!("description: {:?}", wh.description);
                    println!("tech_stack: {:?}", wh.tech_stack);
                    println!("order: {:?}", wh.order);
                }
            }
            "/photos/favs" => {
                let data = get_data::<FavPhotos>(cli.path).await;
                for photo in data.images {
                    println!("-------------------------");
                    println!("uri: {:?}", photo.uri);
                }
            }
            _ => {
                println!("Invalid path");
            }
        },
        _ => {
            println!("Invalid command");
        }
    }
}
