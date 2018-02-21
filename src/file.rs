use std::io::{BufRead, Cursor};
use std::str;
use front_matter::FrontMatter;

#[derive(Debug)]
pub struct File {
    pub front_matter: Option<FrontMatter>,
    pub body: String,
}

impl File {
    pub fn parse_file(utf8: &[u8]) -> Self {
        let (front_matter_utf8, body_utf8) = Self::split(&utf8);

        let front_matter = match FrontMatter::build(front_matter_utf8) {
            Ok(f) => Some(f),
            Err(_) => None,
        };

        File {
            front_matter: front_matter,
            body: str::from_utf8(body_utf8).unwrap().to_owned(),
        }
    }

    fn split(utf8: &[u8]) -> (&[u8], &[u8]) {
        let mut cursor = Cursor::new(utf8);
        let mut buf = String::new();
        let mut last_bytes;

        cursor.read_line(&mut buf).unwrap();

        // Skip over first line
        if buf == String::from("\n") {
            buf.clear();
            cursor.read_line(&mut buf).unwrap();
        }

        if buf == String::from("---\n") {
            let front_matter_start: usize = (cursor.position() - 4) as usize;

            last_bytes = cursor.read_line(&mut buf).unwrap();
            buf.clear();

            while last_bytes != 0 && buf != String::from("---\n") {
                buf.clear();
                last_bytes = cursor.read_line(&mut buf).unwrap();
            }

            if last_bytes != 0 && buf == String::from("---\n") {
                let front_matter_end: usize = cursor.position() as usize;
                return (
                    &utf8[front_matter_start..front_matter_end],
                    &utf8[(front_matter_end - 1) + 1..utf8.len()],
                );
            }
        }

        // Default return for files without front matter
        (&utf8[0..0], &utf8[0..utf8.len()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_test() {
        let file = File::parse_file(
            b"---
title: hi
---
# My first post
",
        );

        assert!(file.front_matter.is_some());
        assert_eq!(file.body, "# My first post\n");
    }

    #[test]
    fn parse_no_front_matter_file_test() {
        let file = File::parse_file(
            b"
# My first post
Lorem ipsum dolor sit amet, consectetur adipisicing elit. 
Voluptas quia perferendis, tenetur optio doloribus repellendus commodi tempora provident, 
sunt quasi ab excepturi. Iusto quasi cupiditate consectetur facere officia rem similique.
",
        );

        assert!(file.front_matter.is_none());
    }

    #[test]
    fn parse_broken_front_matter_file_test() {
        let file = File::parse_file(
            b"---
title: hi
# My first post
Lorem ipsum dolor sit amet, consectetur adipisicing elit. 
Voluptas quia perferendis, tenetur optio doloribus repellendus commodi tempora provident, 
sunt quasi ab excepturi. Iusto quasi cupiditate consectetur facere officia rem similique.
",
        );

        assert!(file.front_matter.is_none());
    }
}
