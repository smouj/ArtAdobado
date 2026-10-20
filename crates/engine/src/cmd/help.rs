//! Help → support and project links (the Adobado Kit website, this app's repository and its
//! GitHub repository). The commands return the URL; the frontend opens it.

use serde_json::{Value, json};

use super::*;

/// This app's public repository name.
pub const APP_ID: &str = "ArtAdobado";
/// Where users can get help or report a problem.
pub const SUPPORT_URL: &str = "https://github.com/smouj/ArtAdobado/issues";
pub const WEBSITE_URL: &str = "https://smouj.github.io/Adobado_Kit/";

/// This app's project page.
pub fn app_page_url() -> String {
    github_url()
}

/// This app's source repository.
pub fn github_url() -> String {
    format!("https://github.com/smouj/{APP_ID}")
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.discord", "Get Help and Report an Issue", ["Help"], None, "{} → {url} the ArtAdobado issue tracker", always, |_, _| url(SUPPORT_URL.into())),
        cmd!(query "help.website", "Adobado Kit Website", ["Help"], None, "{} → {url}", always, |_, _| url(WEBSITE_URL.into())),
        cmd!(query "help.appPage", "ArtAdobado Project Page", ["Help"], None, "{} → {url} this app's repository", always, |_, _| url(app_page_url())),
        cmd!(query "help.github", "ArtAdobado on GitHub", ["Help"], None, "{} → {url} source code and project history", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {support, website, appPage, github}", always, |_, _| {
            Ok(json!({ "support": SUPPORT_URL, "website": WEBSITE_URL, "appPage": app_page_url(), "github": github_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.discord", &json!({})).unwrap()["url"], SUPPORT_URL);
        assert_eq!(s.execute("help.appPage", &json!({})).unwrap()["url"], "https://github.com/smouj/ArtAdobado");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/smouj/ArtAdobado");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["website"], WEBSITE_URL);
    }
}
