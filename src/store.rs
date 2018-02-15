use git2::Repository;

use config::Config;
use utils;

pub struct Store {
    pub repo: Repository,
    pub config: Config,
    branch: String,
}

impl Store {
    pub fn init(repo: Repository) -> Self {
        let branch = "master";

        // Build config -
        // TODO: Currently it breaks on projects without config :/
        // add more graceful handling
        let config_string = utils::read_file(&repo, "config.yml", branch);
        let config = Config::parse(&config_string).unwrap();

        Store {
            repo: repo,
            config: config,
            branch: branch.to_owned(),
        }
    }

    pub fn branch(&self) -> &String {
        &self.branch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::git;
    use test_support::git::DroppableDirectory;
    use tempdir::TempDir;

    #[test]
    fn build_store() {
        let td = TempDir::new("given-test-dir").unwrap();
        let path = td.path();

        let (repo, dir) = git::RepoBuilder::init(path)
            .file(
                "config.yml",
                "{ site_name: Value A, site_url: 66, description: \"67\" }",
            )
            .file("_posts/2018-1-1-my-post.md", "Content of my post")
            .build_bare();

        let store = Store::init(repo);

        assert_eq!("master", store.branch);
        assert_eq!("Value A", store.config.site_name)
    }
}
