mod request;

use serde_json::to_string as json_string;

use self::request::Request;
use Store;
use Post;

fn config_handle(_request: Request, store: &Store) -> Result<String, String> {
    match json_string(&store.config) {
        Ok(result) => Ok(result),
        _ => Err("Getting string".to_owned()),
    }
}

fn posts_handle(request: Request, store: &Store) -> Result<String, String> {
    println!("{:?}", request.resource_params());

    // Left here
    if request.resource_params().is_none() {
        let posts = Post::list_posts().all().execute(&store);

        return match json_string(&posts) {
            Ok(result) => Ok(result),
            _ => Err("Error getting posts".to_owned()),
        };
    } else {
        let post = Post::find("2018-1-1-my-post").execute(&store);

        return match json_string(&post) {
            Ok(result) => Ok(result),
            _ => Err("Error getting post".to_owned()),
        };
    }
}

fn healthcheck_handle(_request: Request, store: &Store) -> Result<String, String> {
    match json_string(&store.health_check()) {
        Ok(result) => Ok(result),
        _ => Err("Cannot parse health check".to_owned()),
    }
}

fn error_handle(request: Request) -> Result<String, String> {
    Err(format!("Cannot find path {}", request.resource()).into())
}

fn debug_handle(request: Request) -> Result<String, String> {
    Ok(format!(
        "config handle for resource: {} and resource_params: {:?} with query_string {:?}",
        request.resource(),
        request.resource_params(),
        request.query_string()
    ).into())
}

pub fn call(request_path: &str, store: &Store) -> Result<String, String> {
    let request: Request = Request::new(&request_path);

    match request.resource() {
        "/config" => config_handle(request, store),
        "/posts" => posts_handle(request, store),
        "/health_check" => healthcheck_handle(request, store),
        "/_debug" => debug_handle(request),
        _ => error_handle(request),
    }
}
