use serde::{Serialize, Deserialize};
use reqwest::{Error, blocking::Client};
use anyhow::{Context, Result};
use unicode_normalization::UnicodeNormalization;
use std::env;

fn main() -> Result<()> {
    let username = env::var("USERNAME").context("Confluence username not found!")?;
    let password = env::var("PASSWORD").context("Confluence password not found!")?;
    let base_url = env::var("BASE_URL").context("Confluence base url not found!")?;

    let args: Vec<String> = env::args().collect();
    let query = &args[1].nfc().collect::<String>();
    let request_url = format!("{base_url}/rest/quicknav/1/search", base_url = base_url);

    let client = Client::new();
    let request = client
        .get(request_url)
        .query(&[("query", query)])
        .basic_auth(username, Some(password));
    let response = request.send()?;

    let result: ApiResponse = response.json()?;

    let result_list = AlfredResultList::from(result, &base_url);
    if cfg!(debug_assertions) {
        println!("{:#?}", result_list);
    }
    let out = serde_json::to_string(&result_list).unwrap();
    println!("{}", out);

    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    query_tokens: Vec<String>,
    query: String,
    total_size: u32,
    content_name_matches: Vec<Vec<Match>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Match {
    id: Option<String>,
    name: String,
    href: String,
    class_name: String,
    space_name: Option<String>,
    space_key: Option<String>,
}

#[derive(Serialize, Debug)]
struct AlfredResult {
    title: String,
    subtitle: String,
    arg: String,
    icon: AlfredResultIcon,
}

#[derive(Serialize, Debug)]
struct AlfredResultIcon {
    path: String,
}

#[derive(Serialize, Debug)]
struct AlfredResultList {
    items: Vec<AlfredResult>,
}

impl AlfredResult {
    fn from(confluence_match: Match, base_url: &String) -> AlfredResult {
        AlfredResult {
            title: html_escape::decode_html_entities(&confluence_match.name).into_owned(),
            subtitle: confluence_match.space_name.unwrap(),
            arg: format!("{}{}", base_url, confluence_match.href),
            icon: AlfredResultIcon {
                path: format!("assets/{}.png", confluence_match.class_name),
            },
        }
    }
}

impl AlfredResultList {
    fn from(response: ApiResponse, base_url: &String) -> AlfredResultList {
        AlfredResultList {
            items: response
                .content_name_matches
                .into_iter()
                .flatten()
                .filter(|m| m.id.is_some())
                .filter(|m| m.class_name == "content-type-page" || m.class_name == "content-type-blogpost" || m.class_name == "search-for")
                .map(|m| AlfredResult::from(m, base_url))
                .collect(),
        }
    }
}
