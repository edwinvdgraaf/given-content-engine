use std::path::Path;
use std::str;
use git2::{Blob, BranchType, Index, Repository, Tree, TreeEntry};

fn find_path_tree_entry<'a>(repo: &'a Repository, tree: &'a Tree, path: &Path) -> Tree<'a> {
    let dir_entry = tree.get_path(&path).unwrap();
    let dir_tree_object = dir_entry.to_object(&repo).unwrap();
    dir_tree_object.into_tree().unwrap()
}

fn find_blob<'a>(repo: &'a Repository, tree: &Tree, filename: &str) -> Blob<'a> {
    let path_tree: Tree;
    let file_tree_entry: TreeEntry;

    if filename.contains("/") {
        let path = Path::new(&filename);
        path_tree = find_path_tree_entry(&repo, &tree, path.parent().unwrap());
        file_tree_entry = path_tree
            .get_name(path.file_name().unwrap().to_str().unwrap())
            .unwrap();
    } else {
        file_tree_entry = tree.get_name(&filename).unwrap();
    }

    let object = file_tree_entry.to_object(&repo).unwrap();
    object.into_blob().unwrap()
}

pub fn read_file_raw<'a>(repo: &'a Repository, filename: &str, branch_name: &str) -> Vec<u8> {
    let blob;
    let branch = repo.find_branch(&branch_name, BranchType::Local).unwrap();
    let branch_ref = branch.get();

    let oid = branch_ref.target().unwrap();

    let tree = match repo.find_commit(oid) {
        Ok(commit) => commit.tree().unwrap(),
        Err(e) => panic!("Cannot get tree for branch: {}", e),
    };

    blob = find_blob(&repo, &tree, filename);
    blob.content().to_vec()
}

pub fn read_file(repo: &Repository, filename: &str, branch_name: &str) -> String {
    let branch = repo.find_branch(&branch_name, BranchType::Local).unwrap();
    let branch_ref = branch.get();

    let oid = branch_ref.target().unwrap();

    let tree = match repo.find_commit(oid) {
        Ok(commit) => commit.tree().unwrap(),
        Err(e) => panic!("Cannot get tree for branch: {}", e),
    };
    let blob = find_blob(&repo, &tree, filename);

    // We know these bytes are valid, so just use `unwrap()`.
    return str::from_utf8(blob.content()).unwrap().to_owned();
}

pub fn read_dir(
    repo: &Repository,
    branch_name: &str,
    option_dir_name: Option<&str>,
) -> Vec<String> {
    let branch = repo.find_branch(&branch_name, BranchType::Local).unwrap();
    let branch_ref = branch.get();

    let oid = branch_ref.target().unwrap();

    let tree = match repo.find_commit(oid) {
        Ok(commit) => commit.tree().unwrap(),
        Err(e) => panic!("Cannot get tree for branch: {}", e),
    };

    let mut memory_index = Index::new().unwrap();
    memory_index.read_tree(&tree).unwrap();

    if option_dir_name == None {
        return memory_index
            .iter()
            .map(|e| String::from_utf8(e.path).unwrap())
            .collect();
    }

    let dir_name = option_dir_name.unwrap().as_bytes();
    return memory_index
        .iter()
        .filter(|e| dir_name == &(e.path[0..dir_name.len()]))
        .map(|e| String::from_utf8(e.path).unwrap())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::git;
    use tempdir::TempDir;

    #[test]
    fn read_file_test() {
        let td = TempDir::new("test").unwrap();
        let path = td.path();

        let repo = git::RepoBuilder::init(path)
            .file("index.md", "Content inside file index.md")
            .file("inside-another/nested-dir/lala.md", "inside lala")
            .file("nested-dir/lala.md", "lala")
            .build();

        assert_eq!(
            "Content inside file index.md",
            read_file(&repo, "index.md", "master")
        );

        assert_eq!("lala", read_file(&repo, "nested-dir/lala.md", "master"));
    }

    #[test]
    fn read_dir_test() {
        let td = TempDir::new("test").unwrap();
        let path = td.path();

        let repo = git::RepoBuilder::init(path)
            .file("index.md", "Content inside file index.md")
            .file(
                "_posts/2018-1-1-my-first-post.md",
                "Content inside of _posts/2018-1-1-my-first-post.md",
            )
            .file(
                "_posts/2018-1-2-my-first-post.md",
                "Content inside of _posts/2018-1-2-my-first-post.md",
            )
            .file(
                "_posts/2018-1-3-my-first-post.md",
                "Content inside of _posts/2018-1-3-my-first-post.md",
            )
            .build();

        let files = read_dir(&repo, "master", Some("_posts"));

        assert_eq!(files.len(), 3);
        assert_eq!(files[0], "_posts/2018-1-1-my-first-post.md");
        assert_eq!(files[1], "_posts/2018-1-2-my-first-post.md");
        assert_eq!(files[2], "_posts/2018-1-3-my-first-post.md");
    }
}
