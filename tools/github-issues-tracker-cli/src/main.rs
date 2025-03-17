use dotenv::dotenv;
use futures::future::BoxFuture;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PullRequest {}

#[derive(Debug, Serialize, Deserialize)]
struct Issue {
    number: usize,
    title: String,
    pull_request: Option<PullRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IssueReaction {
    content: String,
    user: User,
}

fn construct_new_url(headers: &HeaderMap) -> Option<String> {
    headers.get("link").and_then(|link_header| {
        link_header.to_str().ok().and_then(|link_value| {
            link_value.contains("rel=\"next\"").then(|| {
                link_value
                    .split(";")
                    .collect::<Vec<&str>>()
                    .get(0)
                    .expect("Could not find new url with page")
                    .trim_start_matches("<")
                    .trim_end_matches(">")
                    .to_string()
            })
        })
    })
}

async fn get_issues(url: Option<String>) -> Vec<Issue> {
    let token: String = std::env::var("GITHUB_PAT").expect("Expected GITHUB_PAT in env file");
    let request_url = url.unwrap_or(format!(
        "https://api.github.com/repos/{owner}/{repo}/issues?state=open&page=1&per_page=100",
        owner = "taga3s",
        repo = "taga3s.dev-assets"
    ));
    let client: reqwest::Client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(AUTHORIZATION, format!("Bearer {token}", token = token))
        .header(USER_AGENT, "rust web-api")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status().is_success() => res,
        _ => return Vec::new(),
    };

    let new_url = construct_new_url(response.headers());

    let issues = response
        .json::<Vec<Issue>>()
        .await
        .expect("Something went wrong while parsing")
        .into_iter()
        .filter(|issue| issue.pull_request.is_none())
        .collect::<Vec<_>>();

    if let Some(new_url) = new_url {
        let more_issues = get_issues_wrapper(Some(new_url)).await;
        return issues.into_iter().chain(more_issues).collect();
    }

    issues
}

fn get_issues_wrapper(url: Option<String>) -> BoxFuture<'static, Vec<Issue>> {
    Box::pin(get_issues(url))
}

async fn get_issue_reactions(issue_id: usize) -> Vec<IssueReaction> {
    let token: String = std::env::var("GITHUB_PAT").expect("Expected GITHUB_PAT in env file");
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/issues/{issue_id}/reactions",
        owner = "taga3s",
        repo = "taga3s.dev-assets",
        issue_id = issue_id
    );
    let client: reqwest::Client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(AUTHORIZATION, format!("Bearer {token}", token = token))
        .header(USER_AGENT, "rust web-api")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .expect("Something went wrong while fetching");
    let resolved_response = response
        .json::<Vec<IssueReaction>>()
        .await
        .expect("Something went wrong while parsing");
    resolved_response
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let issues = get_issues(None).await;

    for issue in &issues {
        let reactions = get_issue_reactions(issue.number).await;
        println!("Issue: {:?}", issue.title);
        println!("Reactions: {:?}", reactions);
    }

    println!("Amount of issues: {:?}", issues.len());
}
