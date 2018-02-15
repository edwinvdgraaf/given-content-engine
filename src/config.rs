use serde_yaml::{from_str, Error};
use serde_json::to_string as json_string;

fn empty_vec() -> Vec<NavItems> {
    Vec::new()
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub site_name: String,
    pub site_url: String,
    pub description: String,
    #[serde(default = "empty_vec")] pub toc: Vec<NavItems>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NavItems {
    pub title: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub path: Option<String>,
    #[serde(default = "empty_vec")] pub children: Vec<NavItems>,
}

impl Config {
    pub fn parse<'a>(str_config: &str) -> Result<Config, Error> {
        from_str(str_config)
    }

    pub fn to_string(&self) -> String {
        json_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn parse_config_test() {
        // Note site_url is converted to string by serde for us
        let config =
            Config::parse(r#"{ site_name: Value A, site_url: 66, description: "67" }"#).unwrap();

        assert_eq!(config.site_name, "Value A");
        assert_eq!(config.site_url, "66");
        assert_eq!(config.description, "67");
    }

    #[test]
    fn parse_config_with_toc_test() {
        let config = Config::parse(
            r#"
        site_name: My site
        site_url: 66
        description: My site driven by given
        toc: 
          - title: About page
            url: about.html
        "#,
        ).unwrap();

        let toc = config.toc;

        assert_eq!(toc[0].title, "About page");
        assert_eq!(toc[0].url, "about.html");
    }
}
