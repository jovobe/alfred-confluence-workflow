# Alfred Confluence Workflow
Search Confluence from Alfred and open results in your browser.

## Features
- Search Confluence from Alfred and open results in your browser
- Copy Confluence page URL to clipboard (⌘ + C on an Alfred result)
- Works on M1 and Intel Macs
- Authenticate with your Confluence account via Access Token or Username/Password
- Works for Confluence Enterprise and Confluence Cloud

## Usage
1. Download the latest version of the workflow [here](https://github.com/jovobe/alfred-confluence-workflow/releases)
2. Open the workflow in Alfred
3. Adjust the environment variables accordingly
4. Type `c <search term>` in Alfred to search Confluence

### Environment variables
- `BASE_URL`: Confluence base URL e.g. https://confluence.example.com. For Cloud users: Don't forget to append the "/wiki" part here! (e.g. https://example.atlassian.net/wiki)
- `ACCESS_TOKEN`: Confluence access token (only for Confluence Enterprise)
- `USERNAME`: Confluence username
- `PASSWORD`: Confluence password (or API Token when the cloud version is used, see [here](#authentication))

### Authentication
**For Cloud Users:** In the cloud version you have to use the username/password combination! The username is your Confluence username and the password is an API Token. You can obtain a token [here](https://id.atlassian.com/manage/api-tokens).

You can use either an access token or a username/password combination. You can obtain an access token by following the instructions [here](https://confluence.atlassian.com/enterprise/using-personal-access-tokens-1026032365.html). If you use an access token, you can omit the username and password. If you use a username/password combination, you can omit the access token. If you provide both, the access token will be used.

## Development
To compile the source code for Intel or ARM Macs you might need to install the specific rust toolchain. This can be done by running the following command for Intel based toolchain: `rustup target add x86_64-apple-darwin` and the following command for ARM based toolchain: `rustup target add aarch64-apple-darwin`.

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request with your changes or open an issue with your suggestions.

For quality assurance, I personally will review your pull requests and merge them when they are approved. Because I maintain this project in my spare time, it might take some time for me to review your pull requests. Please be patient.

## Credits
This project is heavily inspired by [alfred-confluence](https://github.com/skleinei/alfred-confluence) which is a Python based workflow for searching Confluence. The mentioned project is not working anymore due to the removal of Python 2 support in macOS 12.3.

I would like to give a shoutout and big thanks to all the contributors of this project!

---

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
