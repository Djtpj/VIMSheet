use std::fs;

struct Sheet {
    content: String,
}

impl Sheet {
    pub fn new(path: &str) -> Self {
        Sheet { 
            content: Sheet::read_content(path),
        }
    }

    fn read_content(path: &str) -> String {
       fs::read_to_string(path).expect("Should have been able to read the file, but couldn't") 
    }
}

fn main() {
    let sheet = Sheet::new("C:\\Users\\roark\\Downloads\\addresses.csv");

    println!("{}", sheet.content);
}
