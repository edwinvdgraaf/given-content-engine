extern crate given_content_engine;
extern crate tempdir;
extern crate test_support;

mod integration_test {
    use test_support::git;
    use tempdir::TempDir;
    use given_content_engine::{endpoint, Post, Store};

    #[test]
    fn query_a_list_of_post() {
        let td = TempDir::new("test").unwrap();
        let path = td.path();

        let (repo, _dir) = git::RepoBuilder::init(path)
            .file(
                "config.yml",
                "{ site_name: Value A, site_url: 66, description: \"67\" }",
            )
            .file("index.md", "Content inside file index.md")
            .file("_posts/2018-1-1-my-post.md", "Content of my post")
            .build_bare();

        let store = Store::init(repo);
        let posts = Post::list_posts().all().execute(&store);

        assert_eq!(posts[0].content, "Content of my post");
    }

    #[test]
    fn query_one_post() {
        let td = TempDir::new("test").unwrap();
        let path = td.path();

        let (repo, _dir) = git::RepoBuilder::init(path)
            .file(
                "config.yml",
                "{ site_name: Value A, site_url: 66, description: \"67\" }",
            )
            .file("index.md", "Content inside file index.md")
            .file("_posts/2018-1-1-my-post.md", "Content of my post")
            .build_bare();

        let store = Store::init(repo);
        let post = Post::find("2018-1-1-my-post.md").execute(&store);

        assert_eq!(post.content, "Content of my post");
    }

    #[test]
    fn endpoint_config() {
        let td = TempDir::new("test").unwrap();
        let path = td.path();

        let (repo, _dir) = git::RepoBuilder::init(path)
            .file(
                "config.yml",
                "{ site_name: Value A, site_url: 66, description: \"67\" }",
            )
            .file("index.md", "Content inside file index.md")
            .file("_posts/2018-1-1-my-post.md", "Content of my post")
            .build_bare();

        let store = Store::init(repo);

        assert!(
            endpoint::call("/config", &store)
                .unwrap()
                .contains("site_name")
        );
        assert!(
            endpoint::call("/config", &store)
                .unwrap()
                .contains("site_url")
        );
        assert!(
            endpoint::call("/config", &store)
                .unwrap()
                .contains("description")
        );
        assert!(
            endpoint::call("/config", &store)
                .unwrap()
                .contains("Value A")
        );
    }

}
