use serde_yaml::{from_str, Error};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub site_name: String,
    pub site_url: String,
    pub description: String,
    pub toc: Option<Vec<NavItems>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NavItems {
    pub title: String,
    pub url: String,
    pub path: Option<String>,
    pub children: Option<Vec<NavItems>>,
}

pub fn parse_config<'a>(str_config: &str) -> Result<Config, Error> {
    from_str(str_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn parse_config_test() {
        // Note site_url is converted to string by serde for us
        let config =
            parse_config(r#"{ site_name: Value A, site_url: 66, description: "67" }"#).unwrap();
        assert_eq!(config.site_name, "Value A");
        assert_eq!(config.site_url, "66");
        assert_eq!(config.description, "67");
    }

    #[test]
    fn parse_config_with_toc_test() {
        let config = parse_config(
            r#"
        site_name: My site
        site_url: 66
        description: My site driven by given
        toc: 
          - title: About page
            url: about.html
        "#,
        ).unwrap();

        let toc = config.toc.unwrap();

        assert_eq!(toc[0].title, "About page");
        assert_eq!(toc[0].url, "about.html");
    }
}
