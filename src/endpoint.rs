struct RequestParams {
    key: String,
    value: String,
}

struct Request<'a> {
    path: &'a str,
}

impl<'a> Request<'a> {
    fn new(path: &'a str) -> Self {
        // Add validation
        // starts with one /
        Request { path: &path }
    }

    fn resource(&self) -> &'a str {
        // Find all seperators in path
        let seperator = "/";

        // Cache matches for better speed?
        let matches: Vec<_> = self.path.match_indices(seperator).collect();

        // Set fallback position for only resource paths
        let mut resource_seperator_pos: usize = self.path.len();

        if matches.len() >= 2 {
            resource_seperator_pos = matches[1].0;
        }

        &self.path[0..resource_seperator_pos]
    }

    fn members(&self) -> Option<&'a str> {
        // Find all seperators in path
        let seperator = "/";

        // Cache matches for better speed?
        let matches: Vec<_> = self.path.match_indices(seperator).collect();

        // Only 1 seperator, so no members
        if matches.len() <= 1 {
            return None;
        }

        // Find position from where to get the members
        let resource_seperator_pos = matches[1].0 + 1;

        Some(&self.path[resource_seperator_pos..self.path.len()])
    }
}

fn config_handle(request: Request) -> Result<String, String> {
    Ok(format!(
        "config handle for resource: {} and members: {:?}",
        request.resource(),
        request.members()
    ).into())
}

pub fn call(request_path: &str) -> Result<String, String> {
    let request: Request = Request::new(&request_path);

    match request.resource() {
        "/config" => config_handle(request),
        _ => Err(format!("Cannot find path {}", request.resource()).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_resource_test() {
        let id = "/config";
        let request = Request { path: &id };

        assert_eq!(request.resource(), "/config");
    }

    #[test]
    fn extract_resource_with_member_test() {
        let id = "/posts/12";
        let request = Request { path: &id };

        assert_eq!(request.resource(), "/posts");
        assert_eq!(request.members(), Some("12"));
    }

    #[test]
    fn handles_empty_members_test() {
        // Not sure if this is a really valid case,
        // just, it should panic, when url given
        let id = "/posts/";
        let request = Request { path: &id };

        assert_eq!(request.resource(), "/posts");
        assert_eq!(request.members(), Some(""));
    }
}
