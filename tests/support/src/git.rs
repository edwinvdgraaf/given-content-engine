use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use tempdir::TempDir;
use std::mem;

use git2;

#[derive(Debug)]
pub struct DroppableDirectory {
    pub inner: Option<TempDir>,
    pub path: Option<PathBuf>,
    drop: bool,
}

impl Drop for DroppableDirectory {
    fn drop(&mut self) {
        self.path = None;
        if !self.drop {
            if let Some(dir) = mem::replace(&mut self.inner, None) {
                println!("Not dropping dir for: {:?}", dir.into_path());
            }
        } else {
            // match self.inner {
            //     Some(ref mut dir) => println!("Dropping dir {:?}", dir.path()),
            //     None => panic!("huh what?"),
            // }
        }
    }
}

pub struct RepoBuilder {
    repo: git2::Repository,
    files: Vec<PathBuf>,
    path: PathBuf,
}

pub struct BareRepoBuilder {
    repo: git2::Repository,
}

pub fn repo(p: &Path) -> RepoBuilder {
    RepoBuilder::init(p)
}

pub fn repo_into_bare(clone_path: &PathBuf, into_path: &Path) -> git2::Repository {
    let mut nativeBuilder = git2::build::RepoBuilder::new();
    nativeBuilder.bare(true);

    nativeBuilder
        .clone(clone_path.to_str().unwrap(), into_path)
        .unwrap()
}

impl RepoBuilder {
    pub fn init(p: &Path) -> RepoBuilder {
        t!(fs::create_dir_all(p.parent().unwrap()));
        let repo = t!(git2::Repository::init(p));

        {
            let mut config = t!(repo.config());
            t!(config.set_str("user.name", "name"));
            t!(config.set_str("user.email", "email"));
        }
        RepoBuilder {
            repo: repo,
            files: Vec::new(),
            path: p.to_owned(),
        }
    }

    pub fn file(self, path: &str, contents: &str) -> RepoBuilder {
        let mut me = self.nocommit_file(path, contents);
        me.files.push(PathBuf::from(path));
        me
    }

    pub fn nocommit_file(self, path: &str, contents: &str) -> RepoBuilder {
        let dst = self.repo.workdir().unwrap().join(path);
        t!(fs::create_dir_all(dst.parent().unwrap()));
        t!(t!(File::create(&dst)).write_all(contents.as_bytes()));
        self
    }

    pub fn build(self) -> git2::Repository {
        {
            let mut index = t!(self.repo.index());
            for file in self.files.iter() {
                t!(index.add_path(file));
            }
            t!(index.write());
            let id = t!(index.write_tree());
            let tree = t!(self.repo.find_tree(id));
            let sig = t!(self.repo.signature());
            t!(self.repo
                .commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]));
        }
        let RepoBuilder { repo, .. } = self;
        repo
    }

    pub fn build_bare(self) -> (git2::Repository, DroppableDirectory) {
        self.build_bare_generic(true)
    }

    pub fn build_bare_no_drop(self) -> (git2::Repository, DroppableDirectory) {
        self.build_bare_generic(false)
    }

    fn build_bare_generic(self, drop: bool) -> (git2::Repository, DroppableDirectory) {
        let td_bare_clone = TempDir::new("given-test-dir").unwrap();
        let mut clone_path = td_bare_clone.path().to_path_buf();
        clone_path.push("bare");

        let repo_path_copy = self.path.to_owned();

        let bare_path = DroppableDirectory {
            inner: Some(td_bare_clone),
            drop: drop,
            path: Some(clone_path.to_owned()),
        };

        self.build();

        (repo_into_bare(&repo_path_copy, &clone_path), bare_path)
    }
}
