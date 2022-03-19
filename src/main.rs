use serde::{ Serialize, Deserialize };
use reqwest::{ Error, blocking::Client };
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let request_url = format!(
        "https://{base_url}/rest/quicknav/1/search",
        base_url = "confluence.atlassian.com",
    );

    if let (Ok(username), Ok(password)) = (env::var("USERNAME"), env::var("PASSWORD")) {
        let client = Client::new();
        let request = client
            .get(request_url)
            .query(&[("query", query)])
            .basic_auth(username, Some(password));
        let response = request.send()?;

        let result: ApiResponse = response.json()?;
        //println!("{:#?}", result);

        let result_list = AlfredResultList::from(result);
        //println!("{:#?}", result_list);
        let out = serde_json::to_string(&result_list).unwrap();
        println!("{}", out);
    } else {
        println!("Envs not found!");
        return Ok(());
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    queryTokens: Vec<String>,
    query: String,
    totalSize: u32,
    contentNameMatches: Vec<Vec<Match>>,
}

#[derive(Deserialize, Debug)]
struct Match {
    id: Option<String>,
    name: String,
    href: String,
    className: String,
    spaceName: Option<String>,
    spaceKey: Option<String>,
}

#[derive(Serialize, Debug)]
struct AlfredResult {
    title: String,
    subtitle: String,
    arg: String,
    icon: String,
}

#[derive(Serialize, Debug)]
struct AlfredResultList {
    items: Vec<AlfredResult>,
}

impl AlfredResult {
    fn from(confluence_match: Match) -> AlfredResult {
        AlfredResult {
            title: confluence_match.name,
            subtitle: format!("{} - {}", confluence_match.spaceKey.unwrap(), confluence_match.spaceName.unwrap()),
            arg: format!("https://{}{}", "confluence.atlassian.com", confluence_match.href),
            icon: format!("assets/{}.png", confluence_match.className),
        }
    }
}

impl AlfredResultList {
    fn from(response: ApiResponse) -> AlfredResultList {
        AlfredResultList {
            items: response
                .contentNameMatches
                .into_iter()
                .flatten()
                .filter(|m| m.id.is_some())
                .filter(|m| m.className == "content-type-page" || m.className == "content-type-blogpost" || m.className == "search-for")
                .map(|m| AlfredResult::from(m))
                .collect(),
        }
    }
}
