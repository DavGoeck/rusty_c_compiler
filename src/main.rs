use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::path::Path;
use std::str::pattern::Pattern;
use std::usize;

fn main() {
    let path = Path::new("/home/david/coden/rust/javid/src/example.c");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("File {} not openable, because {}", display, why),
        Ok(file) => file,
    };

    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Err(_) => panic!("Hello"),
        Ok(content) => content,
    };

    let separators = "()[]{}";

    let mut token: Vec<&str> = vec![];
    let mut last_index: usize = 0;

    let mut i = 0;
    while i < content.len() {
        let c: char = content.chars().nth(i).unwrap();
        if c.is_whitespace() {
            let current_token = &content[last_index..i];
            token.push(current_token);

            let mut next_char = c;
            while next_char.is_whitespace() {
                i = i+1;
                next_char = content.chars().nth(i).unwrap();
            }

            last_index = i;
        }
        else if separators.contains(c) {
            let current_token = &content[last_index..i];
            token.push(current_token);
            i = i+1;

            let separator = &content[i..i];
            i = i+1;
        } else {
            i = i+1;
        }
    }

    
}
