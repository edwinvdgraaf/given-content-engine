use utils;
use file::File;
use store::Store;
use front_matter::FrontMatter;

#[derive(Debug, Serialize)]
pub struct Post {
    pub content: String,
    pub meta: PostMetaData,
}

#[derive(Debug, Serialize)]
pub struct PostMetaData {
    tags: Vec<String>,
}

#[derive(Debug)]
pub struct PostQueryBuilder {
    language: String,
    from: u32,
    to: u32,
}

pub struct PostSingleBuilder {
    id: String,
}

impl From<File> for Post {
    fn from(file: File) -> Self {
        let meta: PostMetaData = match file.front_matter {
            Some(f) => f.into(),
            None => PostMetaData { tags: vec![] },
        };

        Post {
            content: file.body,
            meta: meta,
        }
    }
}

impl From<FrontMatter> for PostMetaData {
    fn from(front_matter: FrontMatter) -> Self {
        PostMetaData {
            tags: front_matter.tags,
        }
    }
}

impl Post {
    // Lets think about input here
    // i18n, from, to for pagination,
    pub fn list_posts() -> PostQueryBuilder {
        PostQueryBuilder {
            language: String::from("en"),
            from: 0,
            to: 5,
        }
    }

    pub fn find(id: &str) -> PostSingleBuilder {
        PostSingleBuilder {
            id: String::from(id),
        }
    }

    // Implement this one generic over list and single query post
    pub fn query_list(_query_builder: &PostQueryBuilder, store: &Store) -> Vec<Post> {
        // Init with capactiy if possible
        let files = utils::read_dir(&store.repo, "master", Some("_posts"));

        files
            .into_iter()
            .map(|file_path| {
                File::parse_file(&utils::read_file_raw(&store.repo, &file_path, "master"))
            })
            .map(|file| file.into())
            .collect()
    }

    pub fn query_one(single_builder: &PostSingleBuilder, store: &Store) -> Post {
        let file_path: String = Self::translate_id_to_filepath(&single_builder.id, &store);

        File::parse_file(&utils::read_file_raw(
            &store.repo,
            &file_path,
            store.branch(),
        )).into()
    }

    fn translate_id_to_filepath(file_name: &str, store: &Store) -> String {
        format!("_posts/{}.{}", file_name, store.config.resolve_extension)
    }
}

impl PostSingleBuilder {
    pub fn execute(&self, store: &Store) -> Post {
        Post::query_one(&self, store)
    }
}

impl PostQueryBuilder {
    pub fn execute(&self, store: &Store) -> Vec<Post> {
        Post::query_list(&self, &store)
    }

    pub fn offset(mut self, from: u32, to: u32) -> Self {
        self.from = from;
        self.to = to;
        self
    }

    pub fn all(mut self) -> Self {
        self.from = 0;
        self.to = <u32>::max_value();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use test_support::git;
    use test_support::git::DroppableDirectory;
    use tempdir::TempDir;

    fn build_repo_with_content() -> (Repository, DroppableDirectory) {
        let td = TempDir::new("given-test-dir").unwrap();
        let path = td.path();

        return git::RepoBuilder::init(path)
            .file("index.md", "Content inside file index.md")
            .file(
                "config.yml",
                "{ site_name: Value A, site_url: 66, description: \"67\" }",
            )
            .file(
                "_posts/2018-1-1-my-post.md",
                r#"---
title: Title of my post
tags: 
    - my-first-post
    - my-tags
---
Content of my post"#,
            )
            .build_bare();
    }

    #[test]
    fn list_posts_with_defaults() {
        let (initialized_repo, _dir) = build_repo_with_content();
        let builder = Post::list_posts();
        assert_eq!(builder.language, "en");
        assert_eq!(builder.from, 0);
        assert_eq!(builder.to, 5);

        let store = Store::init(initialized_repo);

        let posts = builder.execute(&store);

        assert_eq!(posts[0].content, "Content of my post");
    }

    #[test]
    fn list_posts_with_ten_offset() {
        let (initialized_repo, _dir) = build_repo_with_content();
        let builder = Post::list_posts().offset(0, 10);
        assert_eq!(builder.from, 0);
        assert_eq!(builder.to, 10);

        let store = Store::init(initialized_repo);

        let posts = builder.execute(&store);

        assert_eq!(posts[0].content, "Content of my post");
    }

    #[test]
    fn list_all_posts() {
        let (initialized_repo, _dir) = build_repo_with_content();
        let builder = Post::list_posts().all();
        assert_eq!(builder.from, 0);
        assert_eq!(builder.to, <u32>::max_value());

        let store = Store::init(initialized_repo);

        let posts = builder.execute(&store);
        let content = &posts[0].content;
        let meta = &posts[0].meta;

        assert_eq!(meta.tags, vec!["my-first-post", "my-tags"]);
        assert_eq!(content, "Content of my post");
    }

    #[test]
    fn find_post_by_identifier() {
        let (initialized_repo, _dir) = build_repo_with_content();
        let store = Store::init(initialized_repo);
        let builder = Post::find("2018-1-1-my-post");

        assert_eq!(builder.id, "2018-1-1-my-post");

        let post = builder.execute(&store);
        assert_eq!(post.content, "Content of my post");
    }
}
