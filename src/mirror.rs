use std::str;
use git2::{Error, Repository};
use std::path::Path;

pub fn create_mirror(repo_url: &str, into: &Path) -> Result<Repository, Error> {
    let repo = Repository::init_bare(into)?;

    // Set origin and fetchspec
    try!(repo.remote_add_fetch("origin", "+refs/*:refs/*"));
    try!(repo.remote_set_url("origin", &repo_url));

    let mut cfg = repo.config()?;

    // Set mirror to true in .git/config
    try!(cfg.set_bool("remote.origin.mirror", true));

    Ok(repo)
}

pub fn update_mirror(path: &Path) -> Result<(), Error> {
    let repo = Repository::open(path).unwrap();

    let mut remote = repo.find_remote("origin").unwrap();

    try!(remote.fetch(&["refs/heads/*:refs/heads/*"], None, None));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn create_mirror_test() {
        let td = TempDir::new("test").unwrap();
        let path = td.path();

        let repo = create_mirror("/my/git/repo/url", path).unwrap();

        assert!(repo.is_bare());

        // Read .git/config file
        let mut file = File::open(path.join("config")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("fetch = +refs/*:refs/*"));
        assert!(contents.contains("mirror = true"));
        assert!(contents.contains("url = /my/git/repo/url"));
    }
}
