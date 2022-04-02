use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;
use anyhow::{Context, Result};
use unicode_normalization::UnicodeNormalization;
use serde_variant::to_variant_name;
use std::env;

fn main() -> Result<()> {
    let access_token = env::var("ACCESS_TOKEN").context("Confluence access token not found!");
    let username = env::var("USERNAME").context("Confluence username not found!");
    let password = env::var("PASSWORD").context("Confluence password not found!");
    let base_url = env::var("BASE_URL").context("Confluence base url not found!")?;

    let args: Vec<String> = env::args().collect();
    let query = &args[1].nfc().collect::<String>();
    let request_url = format!("{base_url}/rest/quicknav/1/search", base_url = base_url);

    let client = Client::new();
    let mut request = client.get(request_url).query(&[("query", query)]);
    if access_token.is_ok() {
        request = request.bearer_auth(access_token.unwrap());
    } else if username.is_ok() && password.is_ok() {
        request = request.basic_auth(username.unwrap(), Some(password.unwrap()));
    } else {
        panic!("No authentication method found!");
    }
    let response = request.send()?;
    let result: ApiResponse = response.json()?;

    let result_list = AlfredResultList::from(result, &base_url);

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
    class_name: MatchClassName,
    space_name: Option<String>,
    space_key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
enum MatchClassName {
    #[serde(rename = "content-type-page")]
    Page,
    #[serde(rename = "content-type-blogpost")]
    BlogPost,
    #[serde(rename = "search-for")]
    SearchFor,
    #[serde(other)]
    Unknown,
}

impl MatchClassName {
    fn to_string(&self) -> String {
        match self {
            MatchClassName::Page | MatchClassName::BlogPost | MatchClassName::SearchFor => to_variant_name(self).unwrap().into(),
            MatchClassName::Unknown => panic!("Unsupported match class name"),
        }
    }
}

#[derive(Serialize, Debug)]
struct AlfredResult {
    uid: String,
    title: String,
    subtitle: String,
    arg: String,
    icon: AlfredResultIcon,
    text: AlfredResultText,
}

#[derive(Serialize, Debug)]
struct AlfredResultIcon {
    path: String,
}

#[derive(Serialize, Debug)]
struct AlfredResultText {
    copy: String,
}

#[derive(Serialize, Debug)]
struct AlfredResultList {
    items: Vec<AlfredResult>,
}

impl AlfredResult {
    fn from(confluence_match: Match, base_url: &String) -> AlfredResult {
        let url = format!("{}{}", base_url, confluence_match.href);
        AlfredResult {
            uid: confluence_match.id.unwrap(),
            title: html_escape::decode_html_entities(&confluence_match.name).into_owned(),
            subtitle: confluence_match.space_name.unwrap(),
            arg: url.clone(),
            icon: AlfredResultIcon {
                path: format!("assets/{}.png", confluence_match.class_name.to_string()),
            },
            text: AlfredResultText {
                copy: url,
            },
        }
    }

    fn from_search_in_confluence_match(confluence_match: Match, base_url: &String) -> AlfredResult {
        let url = format!("{}{}", base_url, confluence_match.href);
        AlfredResult {
            uid: "search-item".to_string(),
            title: html_escape::decode_html_entities(&confluence_match.name).into_owned(),
            subtitle: "Use full Confluence Search".to_string(),
            arg: url.clone(),
            icon: AlfredResultIcon {
                path: format!("assets/{}.png", confluence_match.class_name.to_string()),
            },
            text: AlfredResultText {
                copy: confluence_match.name,
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
                .filter(|m| m.class_name != MatchClassName::Unknown)
                .map(|m|
                    match m.class_name {
                        MatchClassName::Page | MatchClassName::BlogPost => AlfredResult::from(m, base_url),
                        MatchClassName::SearchFor => AlfredResult::from_search_in_confluence_match(m, base_url),
                        _ => panic!("Unsupported match class name"),
                    }
                )
                .collect(),
        }
    }
}
