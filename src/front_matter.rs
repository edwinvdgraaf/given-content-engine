use std::io::{ BufRead, Cursor };
use std::str;
use serde_yaml::{ from_str, Error };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontMatter {
    la: String
}

#[derive(Debug)]
pub struct FrontMatterExtractError {
    description: String
}

#[derive(Debug)]
pub enum FrontMatterError {
    Parse(Error),
    Extract(FrontMatterExtractError),
}

impl From<Error> for FrontMatterError {
    fn from(err: Error) -> FrontMatterError {
        FrontMatterError::Parse(err)
    }
}

impl From<FrontMatterExtractError> for FrontMatterError {
    fn from(err: FrontMatterExtractError) -> FrontMatterError {
        FrontMatterError::Extract(err)
    }
}

impl FrontMatter {

  pub fn build(utf8: &[u8]) -> Result<Self, FrontMatterError> {
    let cursor = Cursor::new(utf8);
    let mut lines_iter = cursor.lines().map(|l| l.unwrap());
    let mut first_line = lines_iter.next();

    if first_line == Some(String::from("\n")) {
      println!("{:?}", "Found new line");
      first_line = lines_iter.next()
    }


    if first_line == Some(String::from("---")) {
        let mut lines = Vec::new();
        let mut next_line = lines_iter.next();

        // Read until the end or untill '---' delimters are found
        while next_line != None && next_line != Some(String::from("---")) {
            lines.push(next_line.unwrap());
            next_line = lines_iter.next();
        }

        if next_line == None {
          return Err(FrontMatterExtractError {
            description: String::from("No ending front matter delimiter found")
           }).map_err(FrontMatterError::Extract)
        }

        return from_str(&lines.join("\n")).map_err(FrontMatterError::Parse)
    } 

    Err(FrontMatterExtractError {
      description: String::from("No starting front matter delimiter found")
    }).map_err(FrontMatterError::Extract)
  }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_test() {
      let matter = FrontMatter::build(b"---\nla: Lorem ipsum\nYo: No clue\n---"  ).unwrap();
      assert_eq!(matter.la, "Lorem ipsum");
      
    }

    #[test]
    fn without_starting_delimiter_test() {
      let matter = FrontMatter::build(b"Lorem ipsum lalala");
      assert!(matter.is_err());
      let err = matter.unwrap_err();
      match err {
        FrontMatterError::Extract(err) => assert_eq!(err.description, "No starting front matter delimiter found"),
        FrontMatterError::Parse(_) => panic!("Expected: FrontMatterError::Extract error")
      }
    }

    #[test]
    fn without_ending_delimiter_test() {
      let matter = FrontMatter::build(b"---\nla: Hi\n");
      assert!(matter.is_err());
      let err = matter.unwrap_err();
      match err {
        FrontMatterError::Extract(err) => assert_eq!(err.description, "No ending front matter delimiter found"),
        FrontMatterError::Parse(_) => panic!("Expected: FrontMatterError::Extract error")
      }
    }

    #[test]
    fn parse_error_test() {
      let matter = FrontMatter::build(b"---\nna: Hi\n---");
      assert!(matter.is_err());
      let err = matter.unwrap_err();
      match err {
        FrontMatterError::Parse(err) => assert!(err.to_string().contains("missing field")),
        FrontMatterError::Extract(_) => panic!("Expected: FrontMatterError::Extract error")
      }
    }
}
