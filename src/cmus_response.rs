use std::collections::HashMap;
use std::process::Command;



pub fn read_response() -> HashMap<String, String> {
    let mut tags: HashMap<String, String> = HashMap::new();

    let output = Command::new("cmus-remote")
        .arg("-Q")
        .output()
        .expect("Failed to get cmus response");

    let stdout = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 output");

    for line in stdout.lines() {
        let mut parts = line.splitn(2, " ").peekable();
        if let Some(first_part) = parts.peek() {

            match *first_part{
                "tag" => {
                    parts.next();
                    let tag = parts.next().unwrap().to_string();
                    let mut tag_split = tag.splitn(2, " ");
                    tags.insert(tag_split.next().unwrap().to_string(), tag_split.next().unwrap().to_string());
                }

                "set" => continue,

                _ => {
                    tags.insert(parts.next().unwrap().to_string(), parts.next().unwrap().to_string());
                }
            }
        }
    }


    tags
}
