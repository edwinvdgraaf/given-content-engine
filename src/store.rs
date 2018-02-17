use git2::Repository;

use config::Config;
use utils;

pub struct Store {
    pub repo: Repository,
    pub config: Config,
    branch: String,
}

#[derive(Debug, Serialize)]
pub struct StoreHealthCheck {
    sha_reference: String,
    branch: String,
    posts: usize,
    total_files: usize,
    warnings: Vec<String>,
    errors: Vec<String>,
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

    pub fn health_check(&self) -> StoreHealthCheck {
        let posts = utils::read_dir(&self.repo, &self.branch, Some("_posts")).len();
        let files = utils::read_dir(&self.repo, &self.branch, None).len();
        let sha_reference = &self.repo.head().unwrap().target().unwrap();

        let mut warnings = vec![];
        if !(&self.repo.is_bare()) {
            warnings.push("Repo is not in bare mode.".to_owned());
        }

        let errors = vec![];

        StoreHealthCheck {
            sha_reference: format!("{}", sha_reference),
            branch: self.branch.to_owned(),
            posts: posts,
            total_files: files,
            warnings: warnings,
            errors: errors,
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
    use tempdir::TempDir;

    #[test]
    fn build_store() {
        let td = TempDir::new("given-test-dir").unwrap();
        let path = td.path();

        let (repo, _dir) = git::RepoBuilder::init(path)
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

    #[test]
    fn health_check_store() {
        let td = TempDir::new("given-test-dir").unwrap();
        let path = td.path();

        let (repo, _dir) = git::RepoBuilder::init(path)
            .file(
                "config.yml",
                "{ site_name: Value A, site_url: 66, description: \"67\" }",
            )
            .file("_posts/2018-1-1-my-post.md", "Content of my post")
            .build_bare();

        let store = Store::init(repo);
        let status = store.health_check();

        assert!(status.warnings.is_empty());
        assert!(status.errors.is_empty());

        assert_eq!(status.branch, "master");
        assert_eq!(status.posts, 1);
        assert_eq!(status.total_files, 2);
    }
}
