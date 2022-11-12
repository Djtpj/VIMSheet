use crate::mapline::MapLine;
use std::fs;

pub struct Sheet {
    lines: Vec<MapLine>
}

impl Sheet {
    pub fn new(path: String) -> Self {
        let content: String = read_content(path);
        
        let string_lines = parse_string_lines(&content)
            .into_iter()
            .filter(|s| {
                // Make sure this isn't an empty or blank string
                if s.replace(" ", "").is_empty() { return false; }

                is_mappable(s)
            })
            .filter(|s| !s.contains("!exclude"))
            .collect::<Vec<String>>();

        let lines: Vec<MapLine> = map_lines(string_lines);

        Sheet { 
            lines,
        }
    }

    pub fn serialize(self) -> String {
        let mut serialized: Vec<[String; 3]> = Vec::new();

        let mut result = String::new();

        let heading = String::from("Description, Trigger, Mode\n");
        result.push_str(&heading);

        for line in self.lines {
            serialized.push(line.serialize());
        }

        for line in serialized {
            for info in line {
                result.push_str(&info.as_str());
                result.push(',');
            }

            result.push('\n');
        }

        result
    }
}

pub fn parse_string_lines(content: &String) -> Vec<String> {
    content.lines().map(|s| s.to_string()).collect()
}

fn read_content(path: String) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file, but couldn't") 
}

pub fn get_first_word(base: &String) -> Option<String> {
    let split = base.split_whitespace().filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<String>>();

    split.get(0).map(|s| Some(s.clone())).unwrap_or(None).clone()
}

pub fn is_mappable(line: &String) -> bool {
    let word = get_first_word(line);

    if word != None {
        word.unwrap().trim().ends_with("map")
    }
    else {
        false
    }
}

pub fn map_lines(lines: Vec<String>) -> Vec<MapLine> {
    let mut map_lines: Vec<MapLine> = Vec::new();

    for line in lines {
        if line.is_empty() { continue; }

        map_lines.push(MapLine::new(line));
    }

    map_lines
}
