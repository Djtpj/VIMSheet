use std::{fs, io};

mod mapline;

use mapline::MapLine;

struct Sheet {
    content: String,
    lines: Vec<MapLine>
}

impl Sheet {
    pub fn new(path: String) -> Self {
        let content: String = read_content(path);
        let lines: Vec<MapLine> = Vec::new();

        let string_lines = 
            parse_string_lines(&content)
            // Filter Into map declarations
            .into_iter()
            .filter(|s| !s.is_empty())
            .filter(|s| get_first_word(s).ends_with("map"))
            .collect::<Vec<_>>();

        for s in string_lines.clone() {
            println!("Line: {}", s);
        }

        Sheet { 
            content,
            lines,
        }
    }
}



fn main() {
    let input = query_path();

    let sheet = Sheet::new(input);
}

fn query_path() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    return String::from(input.trim());
}

fn parse_string_lines(content: &String) -> Vec<String> {
    content.lines().map(|s| s.to_string()).collect()
}

fn read_content(path: String) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file, but couldn't") 
}

fn get_first_word(base: &String) -> String {
    let split = base.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

    split.get(0).expect("String empty").clone()
}
